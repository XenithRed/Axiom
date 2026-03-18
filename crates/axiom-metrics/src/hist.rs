use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

const BUCKETS: &[f64] = &[
    0.001, 0.005, 0.01, 0.025, 0.05, 0.1,
    0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];

#[derive(Debug)]
struct Inner {
    a: Vec<AtomicU64>,
    b: AtomicU64,
    c: AtomicU64,
}

#[derive(Debug, Clone)]
pub struct Hist {
    a: Arc<Inner>,
    b: &'static str,
    c: &'static str,
}

impl Hist {
    pub fn new(name: &'static str, help: &'static str) -> Self {
        let buckets: Vec<AtomicU64> = (0..BUCKETS.len() + 1)
            .map(|_| AtomicU64::new(0))
            .collect();
        Self {
            a: Arc::new(Inner {
                a: buckets,
                b: AtomicU64::new(0),
                c: AtomicU64::new(0),
            }),
            b: name,
            c: help,
        }
    }

    pub fn observe(&self, v: f64) {
        let bits = v.to_bits();
        self.a.b.fetch_add(1, Ordering::Relaxed);
        self.a.c.fetch_add(bits, Ordering::Relaxed);
        for (i, &upper) in BUCKETS.iter().enumerate() {
            if v <= upper {
                self.a.a[i].fetch_add(1, Ordering::Relaxed);
            }
        }
        self.a.a[BUCKETS.len()].fetch_add(1, Ordering::Relaxed);
    }

    pub fn observe_ms(&self, ms: u64) {
        self.observe(ms as f64 / 1000.0);
    }

    pub fn count(&self) -> u64 { self.a.b.load(Ordering::Relaxed) }

    pub fn sum(&self) -> f64 {
        f64::from_bits(self.a.c.load(Ordering::Relaxed))
    }

    pub fn name(&self) -> &'static str { self.b }
    pub fn help(&self) -> &'static str { self.c }

    pub fn prometheus_lines(&self) -> String {
        let mut out = format!(
            "# HELP {n} {h}\n# TYPE {n} histogram\n",
            n = self.b, h = self.c,
        );
        for (i, &upper) in BUCKETS.iter().enumerate() {
            let v = self.a.a[i].load(Ordering::Relaxed);
            out.push_str(&format!("{}_bucket{{le=\"{upper}\"}} {v}\n", self.b));
        }
        let inf = self.a.a[BUCKETS.len()].load(Ordering::Relaxed);
        out.push_str(&format!("{}_bucket{{le=\"+Inf\"}} {inf}\n", self.b));
        out.push_str(&format!("{}_count {}\n", self.b, self.count()));
        out.push_str(&format!("{}_sum {}\n", self.b, self.sum()));
        out
    }
}
