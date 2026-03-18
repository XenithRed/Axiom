use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Gauge {
    a: Arc<AtomicI64>,
    b: &'static str,
    c: &'static str,
}

impl Gauge {
    pub fn new(name: &'static str, help: &'static str) -> Self {
        Self { a: Arc::new(AtomicI64::new(0)), b: name, c: help }
    }

    #[inline]
    pub fn set(&self, v: i64) { self.a.store(v, Ordering::Relaxed); }

    #[inline]
    pub fn inc(&self) { self.a.fetch_add(1, Ordering::Relaxed); }

    #[inline]
    pub fn dec(&self) { self.a.fetch_sub(1, Ordering::Relaxed); }

    #[inline]
    pub fn add(&self, n: i64) { self.a.fetch_add(n, Ordering::Relaxed); }

    #[inline]
    pub fn get(&self) -> i64 { self.a.load(Ordering::Relaxed) }

    pub fn name(&self) -> &'static str { self.b }
    pub fn help(&self) -> &'static str { self.c }

    pub fn prometheus_line(&self) -> String {
        format!(
            "# HELP {n} {h}\n# TYPE {n} gauge\n{n} {v}\n",
            n = self.b,
            h = self.c,
            v = self.get(),
        )
    }
}
