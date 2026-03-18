pub mod cache;
pub mod error;
pub mod mojang;
pub mod msa;
pub mod token;
pub mod xbl;
pub mod xsts;
pub use error::{Err, R};
pub use token::{McTok, MsaTok, Profile, Session, XblTok, XstsTok};
pub use xsts::{RP_BEDROCK, RP_JAVA};
use reqwest::Client;
use tracing::{info, warn};

pub struct Authenticator {
    cx: Client,
}

impl Authenticator {
    pub fn new() -> R<Self> {
        let cx = Client::builder()
            .https_only(true)
            .use_rustls_tls()
            .build()?;
        Ok(Self { cx })
    }

    pub async fn auth<F>(&self, on_code: F) -> R<Session>
    where
        F: FnOnce(&str, &str),
    {
        if let Some(s) = cache::load()? {
            if s.live() {
                info!("using cached session for {}", s.profile.b);
                return Ok(s);
            }
            info!("session expired, attempting silent refresh");
            if let Ok(s) = self.refresh_session(s).await {
                return Ok(s);
            }
            warn!("silent refresh failed, falling back to device-code flow");
        }
        self.device_flow(on_code).await
    }

    pub async fn force_login<F>(&self, on_code: F) -> R<Session>
    where
        F: FnOnce(&str, &str),
    {
        self.device_flow(on_code).await
    }

    pub fn logout(&self) -> R<()> {
        cache::clear()
    }

    async fn device_flow<F>(&self, on_code: F) -> R<Session>
    where
        F: FnOnce(&str, &str),
    {
        let (dc, uc, url) = msa::begin(&self.cx).await?;
        on_code(&uc, &url);
        let msa = msa::poll(&self.cx, &dc, None).await?;
        let xbl = xbl::auth(&self.cx, &msa.a).await?;
        let xsts = xsts::auth(&self.cx, &xbl.a, RP_JAVA).await?;
        let mc      = mojang::login(&self.cx, &xsts.hdr()).await?;
        let profile = mojang::profile(&self.cx, &mc.a).await?;
        let s = Session { msa, xbl, xsts, mc, profile };
        cache::store(&s)?;
        info!("authenticated as {}", s.profile.b);
        Ok(s)
    }

    async fn refresh_session(&self, old: Session) -> R<Session> {
        let msa = msa::refresh(&self.cx, &old.msa.b).await?;
        let xbl = xbl::auth(&self.cx, &msa.a).await?;
        let xsts = xsts::auth(&self.cx, &xbl.a, RP_JAVA).await?;
        let mc   = mojang::login(&self.cx, &xsts.hdr()).await?;
        let profile = mojang::profile(&self.cx, &mc.a).await?;
        let s = Session { msa, xbl, xsts, mc, profile };
        cache::store(&s)?;
        Ok(s)
    }

    pub async fn xsts_bedrock(&self, s: &Session) -> R<XstsTok> {
        xsts::auth(&self.cx, &s.xbl.a, RP_BEDROCK).await
    }
}

impl Default for Authenticator {
    fn default() -> Self {
        Self::new().expect("failed to build HTTP client")
    }
}
