use std::sync::atomic::{AtomicU32, Ordering};

static NEXT: AtomicU32 = AtomicU32::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Eid(pub u32);

impl Eid {
    pub fn next() -> Self {
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }

    pub fn raw(self) -> u32 { self.0 }
}

impl std::fmt::Display for Eid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E{}", self.0)
    }
}
