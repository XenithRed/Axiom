use ahash::AHashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropMap(Vec<(String, String)>);

impl PropMap {
    pub fn new(props: Vec<(String, String)>) -> Self {
        let mut v = props;
        v.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        Self(v)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }

    pub fn facing(&self) -> Option<&str> { self.get("facing") }
    pub fn half(&self)   -> Option<&str> { self.get("half") }
    pub fn open(&self)   -> Option<bool> { self.get("open").map(|v| v == "true") }
    pub fn powered(&self) -> Option<bool> { self.get("powered").map(|v| v == "true") }
    pub fn waterlogged(&self) -> bool {
        self.get("waterlogged").map(|v| v == "true").unwrap_or(false)
    }
    pub fn lit(&self) -> Option<bool> { self.get("lit").map(|v| v == "true") }
    pub fn snowy(&self) -> Option<bool> { self.get("snowy").map(|v| v == "true") }
    pub fn age(&self) -> Option<u8> {
        self.get("age").and_then(|v| v.parse().ok())
    }
    pub fn level(&self) -> Option<u8> {
        self.get("level").and_then(|v| v.parse().ok())
    }

    pub fn as_slice(&self) -> &[(String, String)] { &self.0 }
}

pub fn remap_facing_java_to_bedrock(f: &str) -> &'static str {
    match f {
        "north" => "north",
        "south" => "south",
        "east"  => "east",
        "west"  => "west",
        "up"    => "up",
        "down"  => "down",
        _       => "north",
    }
}

pub fn remap_half_java_to_bedrock(h: &str) -> &'static str {
    match h {
        "top"    => "top",
        "bottom" => "bottom",
        "upper"  => "top",
        "lower"  => "bottom",
        _        => "bottom",
    }
}
