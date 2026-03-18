use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Counter {
    a: Arc<AtomicU64>,
    b: &'static str,
    c: &'static str,
}

impl Counter {
    pub fn new(name: &'static str, help: &'static str) -> Self {
        Self { a: Arc::new(AtomicU64::new(0)), b: name, c: help }
    }

    #[inline]
    pub fn inc(&self) { self.a.fetch_add(1, Ordering::Relaxed); }

    #[inline]
    pub fn add(&self, n: u64) { self.a.fetch_add(n, Ordering::Relaxed); }

    #[inline]
    pub fn get(&self) -> u64 { self.a.load(Ordering::Relaxed) }

    pub fn name(&self) -> &'static str { self.b }
    pub fn help(&self) -> &'static str { self.c }

    pub fn prometheus_line(&self) -> String {
        format!(
            "# HELP {n} {h}\n# TYPE {n} counter\n{n} {v}\n",
            n = self.b,
            h = self.c,
            v = self.get(),
        )
    }
}
