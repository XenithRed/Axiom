use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCfg {
    pub a: String,
    pub b: bool,
    pub c: Option<String>,
    pub d: bool,
}

impl Default for LogCfg {
    fn default() -> Self {
        Self {
            a: "info".into(),
            b: true,
            c: None,
            d: false,
        }
    }
}

impl LogCfg {
    pub fn level(&self)    -> &str        { &self.a }
    pub fn color(&self)    -> bool        { self.b }
    pub fn file(&self)     -> Option<&str> { self.c.as_deref() }
    pub fn json(&self)     -> bool        { self.d }
}
