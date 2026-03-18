use std::time::Duration;

const A: f64 = 0.125;
const B: f64 = 0.25;
const C: f64 = 0.1;
const D: u64  = 200;

pub struct Rtt {
    a: f64,
    b: f64,
    c: bool,
}

impl Rtt {
    pub fn new() -> Self {
        Self { a: 0.0, b: 0.0, c: false }
    }

    pub fn update(&mut self, s: Duration) {
        let x = s.as_secs_f64();
        if !self.c {
            self.a = x;
            self.b = x / 2.0;
            self.c = true;
        } else {
            let d = (self.a - x).abs();
            self.b = (1.0 - B) * self.b + B * d;
            self.a = (1.0 - A) * self.a + A * x;
        }
    }

    pub fn rto(&self) -> Duration {
        if !self.c {
            return Duration::from_millis(D);
        }
        Duration::from_secs_f64((self.a + 4.0 * self.b).max(C))
    }

    pub fn srtt(&self) -> Duration {
        Duration::from_secs_f64(self.a)
    }
}

impl Default for Rtt {
    fn default() -> Self { Self::new() }
}
