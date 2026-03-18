use crate::{
    error::{Err, R},
    token::Session,
};
use std::path::PathBuf;
use tracing::{debug, warn};

fn cache_path() -> PathBuf {
    let base = dirs::cache_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".cache"));
    base.join("axiom").join("session.json")
}

pub fn load() -> R<Option<Session>> {
    let p = cache_path();
    if !p.exists() {
        debug!("no cache file at {}", p.display());
        return Ok(None);
    }

    let raw = std::fs::read_to_string(&p)
        .map_err(|e| Err::Cache(e.to_string()))?;

    match serde_json::from_str::<Session>(&raw) {
        Ok(s)  => { debug!("session loaded from cache"); Ok(Some(s)) }
        Err(e) => {
            warn!("cache corrupt ({}), ignoring", e);
            let _ = std::fs::remove_file(&p);
            Ok(None)
        }
    }
}

pub fn store(s: &Session) -> R<()> {
    let p = cache_path();
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| Err::Cache(e.to_string()))?;
    }

    let json = serde_json::to_string_pretty(s)?;
    let tmp = p.with_extension("tmp");
    std::fs::write(&tmp, &json).map_err(|e| Err::Cache(e.to_string()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o600))
            .map_err(|e| Err::Cache(e.to_string()))?;
    }

    std::fs::rename(&tmp, &p).map_err(|e| Err::Cache(e.to_string()))?;
    debug!("session cached at {}", p.display());
    Ok(())
}

pub fn clear() -> R<()> {
    let p = cache_path();
    if p.exists() {
        std::fs::remove_file(&p).map_err(|e| Err::Cache(e.to_string()))?;
        debug!("session cache cleared");
    }
    Ok(())
}
