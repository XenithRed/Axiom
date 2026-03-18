use crate::cap::Dir;
use crate::parse::ParsedPkt;

#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub a: Option<Dir>,
    pub b: Option<i32>,
    pub c: Option<String>,
    pub d: Option<usize>,
    pub e: Option<usize>,
}

impl Filter {
    pub fn new() -> Self { Self::default() }

    pub fn dir(mut self, d: Dir) -> Self { self.a = Some(d); self }
    pub fn id(mut self, id: i32) -> Self { self.b = Some(id); self }
    pub fn name(mut self, n: impl Into<String>) -> Self { self.c = Some(n.into().to_lowercase()); self }
    pub fn min_bytes(mut self, n: usize) -> Self { self.d = Some(n); self }
    pub fn max_bytes(mut self, n: usize) -> Self { self.e = Some(n); self }

    pub fn matches(&self, p: &ParsedPkt) -> bool {
        if let Some(d) = self.a { if p.b != d { return false; } }
        if let Some(id) = self.b { if p.c != id { return false; } }
        if let Some(ref n) = self.c {
            if !p.d.to_lowercase().contains(n.as_str()) { return false; }
        }
        if let Some(min) = self.d { if p.e < min { return false; } }
        if let Some(max) = self.e { if p.e > max { return false; } }
        true
    }
}

pub fn parse_filter(args: &crate::Args) -> Filter {
    let mut f = Filter::new();
    if let Some(ref d) = args.direction {
        f = match d.to_lowercase().as_str() {
            "c2s" => f.dir(Dir::C2S),
            "s2c" => f.dir(Dir::S2C),
            _ => f,
        };
    }
    if let Some(id) = args.packet_id {
        f = f.id(id);
    }
    if let Some(ref n) = args.name {
        f = f.name(n.clone());
    }
    if let Some(min) = args.min_bytes {
        f = f.min_bytes(min);
    }
    if let Some(max) = args.max_bytes {
        f = f.max_bytes(max);
    }
    f
}
