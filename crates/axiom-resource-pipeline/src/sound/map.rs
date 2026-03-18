use std::collections::HashMap;
use serde_json::Value;
use crate::error::R;

pub fn convert(raw: &[u8]) -> R<Vec<u8>> {
    let java: Value = serde_json::from_slice(raw)?;
    let mut defs: HashMap<String, Value> = HashMap::new();

    if let Some(obj) = java.as_object() {
        for (k, v) in obj {
            let sounds: Vec<Value> = match v["sounds"].as_array() {
                Some(arr) => arr.iter().filter_map(|s| {
                    let name = s.as_str()
                        .map(|n| n.to_string())
                        .or_else(|| s["name"].as_str().map(|n| n.to_string()))?;
                    let volume = s["volume"].as_f64().unwrap_or(1.0);
                    let pitch  = s["pitch"].as_f64().unwrap_or(1.0);
                    Some(serde_json::json!({
                        "name":   format!("sounds/{name}"),
                        "volume": volume,
                        "pitch":  pitch,
                        "stream": false,
                    }))
                }).collect(),
                None => vec![],
            };

            if sounds.is_empty() { continue; }

            let subtitle = v["subtitle"].as_str().map(|s| s.to_string());
            let mut entry = serde_json::json!({ "sounds": sounds });
            if let Some(st) = subtitle {
                entry["__use_legacy_max_distance__"] = serde_json::json!(false);
            }

            defs.insert(k.clone(), entry);
        }
    }

    let out = serde_json::json!({ "sound_definitions": defs });
    Ok(serde_json::to_string_pretty(&out)?.into_bytes())
}

pub fn extract_ogg_paths(files: &HashMap<String, Vec<u8>>) -> Vec<String> {
    files.keys()
        .filter(|k| k.ends_with(".ogg"))
        .map(|k| k.replace("assets/minecraft/sounds/", "sounds/"))
        .collect()
}
