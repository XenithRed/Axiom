pub mod counter;
pub mod export;
pub mod gauge;
pub mod hist;

pub use counter::Counter;
pub use export::{render, render_json};
pub use gauge::Gauge;
pub use hist::Hist;

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Registry {
    a: Mutex<Vec<Counter>>,
    b: Mutex<Vec<Gauge>>,
    c: Mutex<Vec<Hist>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            a: Mutex::new(Vec::new()),
            b: Mutex::new(Vec::new()),
            c: Mutex::new(Vec::new()),
        }
    }

    pub fn counter(&self, name: &'static str, help: &'static str) -> Counter {
        let c = Counter::new(name, help);
        self.a.lock().unwrap().push(c.clone());
        c
    }

    pub fn gauge(&self, name: &'static str, help: &'static str) -> Gauge {
        let g = Gauge::new(name, help);
        self.b.lock().unwrap().push(g.clone());
        g
    }

    pub fn hist(&self, name: &'static str, help: &'static str) -> Hist {
        let h = Hist::new(name, help);
        self.c.lock().unwrap().push(h.clone());
        h
    }

    pub fn counters(&self) -> Vec<Counter> { self.a.lock().unwrap().clone() }
    pub fn gauges(&self)   -> Vec<Gauge>   { self.b.lock().unwrap().clone() }
    pub fn histograms(&self) -> Vec<Hist>  { self.c.lock().unwrap().clone() }

    pub fn render(&self)      -> String { render(self) }
    pub fn render_json(&self) -> String { render_json(self) }
}

impl Default for Registry { fn default() -> Self { Self::new() } }

pub static GLOBAL: Lazy<Registry> = Lazy::new(Registry::new);

pub mod global {
    use super::*;

    pub static PLAYERS_ONLINE:      Lazy<Gauge>   = Lazy::new(|| GLOBAL.gauge("axiom_players_online",      "Current number of connected players"));
    pub static BEDROCK_PLAYERS:     Lazy<Gauge>   = Lazy::new(|| GLOBAL.gauge("axiom_bedrock_players",     "Bedrock Edition players online"));
    pub static JAVA_PLAYERS:        Lazy<Gauge>   = Lazy::new(|| GLOBAL.gauge("axiom_java_players",        "Java Edition players online"));
    pub static PACKETS_C2S:         Lazy<Counter> = Lazy::new(|| GLOBAL.counter("axiom_packets_c2s_total", "Total client-to-server packets processed"));
    pub static PACKETS_S2C:         Lazy<Counter> = Lazy::new(|| GLOBAL.counter("axiom_packets_s2c_total", "Total server-to-client packets processed"));
    pub static BYTES_RX:            Lazy<Counter> = Lazy::new(|| GLOBAL.counter("axiom_bytes_rx_total",    "Total bytes received"));
    pub static BYTES_TX:            Lazy<Counter> = Lazy::new(|| GLOBAL.counter("axiom_bytes_tx_total",    "Total bytes sent"));
    pub static TRANSLATION_ERRORS:  Lazy<Counter> = Lazy::new(|| GLOBAL.counter("axiom_translation_errors_total", "Block/entity/item translation failures"));
    pub static CHUNK_TRANSLATIONS:  Lazy<Counter> = Lazy::new(|| GLOBAL.counter("axiom_chunk_translations_total",  "Chunk sections translated"));
    pub static PACKET_LATENCY_MS:   Lazy<Hist>    = Lazy::new(|| GLOBAL.hist("axiom_packet_latency_ms",    "Packet processing latency in milliseconds"));
    pub static CHUNK_LATENCY_MS:    Lazy<Hist>    = Lazy::new(|| GLOBAL.hist("axiom_chunk_latency_ms",     "Chunk translation latency in milliseconds"));
    pub static SESSIONS_TOTAL:      Lazy<Counter> = Lazy::new(|| GLOBAL.counter("axiom_sessions_total",    "Total sessions opened since start"));
    pub static ACTIVE_SESSIONS:     Lazy<Gauge>   = Lazy::new(|| GLOBAL.gauge("axiom_active_sessions",     "Currently active bridge sessions"));
    pub static UPTIME_SECS:         Lazy<Gauge>   = Lazy::new(|| GLOBAL.gauge("axiom_uptime_seconds",      "Seconds since Axiom started"));

    pub fn init() {
        Lazy::force(&PLAYERS_ONLINE);
        Lazy::force(&BEDROCK_PLAYERS);
        Lazy::force(&JAVA_PLAYERS);
        Lazy::force(&PACKETS_C2S);
        Lazy::force(&PACKETS_S2C);
        Lazy::force(&BYTES_RX);
        Lazy::force(&BYTES_TX);
        Lazy::force(&TRANSLATION_ERRORS);
        Lazy::force(&CHUNK_TRANSLATIONS);
        Lazy::force(&PACKET_LATENCY_MS);
        Lazy::force(&CHUNK_LATENCY_MS);
        Lazy::force(&SESSIONS_TOTAL);
        Lazy::force(&ACTIVE_SESSIONS);
        Lazy::force(&UPTIME_SECS);
    }
}
