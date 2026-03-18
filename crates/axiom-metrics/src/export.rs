use crate::{Counter, Gauge, Hist, Registry};

pub fn render(reg: &Registry) -> String {
    let mut out = String::new();

    for c in reg.counters() {
        out.push_str(&c.prometheus_line());
        out.push('\n');
    }
    for g in reg.gauges() {
        out.push_str(&g.prometheus_line());
        out.push('\n');
    }
    for h in reg.histograms() {
        out.push_str(&h.prometheus_lines());
        out.push('\n');
    }

    out
}

pub fn render_json(reg: &Registry) -> String {
    let mut obj = serde_json::Map::new();

    for c in reg.counters() {
        obj.insert(c.name().to_string(), serde_json::json!(c.get()));
    }
    for g in reg.gauges() {
        obj.insert(g.name().to_string(), serde_json::json!(g.get()));
    }
    for h in reg.histograms() {
        obj.insert(h.name().to_string(), serde_json::json!({
            "count": h.count(),
            "sum":   h.sum(),
        }));
    }

    serde_json::to_string_pretty(&serde_json::Value::Object(obj))
        .unwrap_or_default()
}
