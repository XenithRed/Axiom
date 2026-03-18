use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpCfg {
    pub a: bool,
    pub b: String,
    pub c: bool,
    pub d: bool,
    pub e: Option<String>,
}

impl Default for RpCfg {
    fn default() -> Self {
        Self {
            a: true,
            b: ".axiom/rp-cache".into(),
            c: true,
            d: true,
            e: None,
        }
    }
}

impl RpCfg {
    pub fn auto_convert(&self)  -> bool        { self.a }
    pub fn cache_dir(&self)     -> &str        { &self.b }
    pub fn convert_models(&self) -> bool       { self.c }
    pub fn convert_sounds(&self) -> bool       { self.d }
    pub fn custom_pack(&self)   -> Option<&str> { self.e.as_deref() }
}
