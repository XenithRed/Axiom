use std::collections::HashMap;

pub fn remap_paths(files: &HashMap<String, Vec<u8>>) -> HashMap<String, Vec<u8>> {
    let mut out = HashMap::new();
    for (path, data) in files {
        let rel = path.replace('\\', "/");
        if let Some(dst) = remap_one(&rel) {
            out.insert(dst, data.clone());
        }
    }
    out
}

fn remap_one(rel: &str) -> Option<String> {
    if !rel.ends_with(".png") { return None; }

    let lower = rel.to_lowercase();
    let java_prefix = "assets/minecraft/textures/";

    if !lower.starts_with(java_prefix) { return None; }

    let after = &rel[java_prefix.len()..];

    let bedrock = if after.starts_with("block/") {
        format!("textures/blocks/{}", &after["block/".len()..])
    } else if after.starts_with("item/") {
        format!("textures/items/{}", &after["item/".len()..])
    } else if after.starts_with("entity/") {
        format!("textures/entity/{}", &after["entity/".len()..])
    } else if after.starts_with("environment/") {
        format!("textures/environment/{}", &after["environment/".len()..])
    } else if after.starts_with("gui/") {
        format!("textures/ui/{}", &after["gui/".len()..])
    } else if after.starts_with("particle/") {
        format!("textures/particle/{}", &after["particle/".len()..])
    } else {
        format!("textures/{after}")
    };

    Some(bedrock)
}

pub fn terrain_texture_json(mapped: &HashMap<String, Vec<u8>>) -> Vec<u8> {
    let mut textures = serde_json::Map::new();
    for path in mapped.keys() {
        if path.starts_with("textures/blocks/") && path.ends_with(".png") {
            let name = path
                .trim_start_matches("textures/blocks/")
                .trim_end_matches(".png");
            textures.insert(name.to_string(), serde_json::json!({
                "textures": path.trim_end_matches(".png")
            }));
        }
    }
    let json = serde_json::json!({
        "resource_pack_name": "axiom_converted",
        "texture_name": "atlas.terrain",
        "texture_data": textures,
    });
    serde_json::to_string_pretty(&json).unwrap_or_default().into_bytes()
}

pub fn item_texture_json(mapped: &HashMap<String, Vec<u8>>) -> Vec<u8> {
    let mut textures = serde_json::Map::new();
    for path in mapped.keys() {
        if path.starts_with("textures/items/") && path.ends_with(".png") {
            let name = path
                .trim_start_matches("textures/items/")
                .trim_end_matches(".png");
            textures.insert(name.to_string(), serde_json::json!({
                "textures": path.trim_end_matches(".png")
            }));
        }
    }
    let json = serde_json::json!({
        "resource_pack_name": "axiom_converted",
        "texture_name": "atlas.items",
        "texture_data": textures,
    });
    serde_json::to_string_pretty(&json).unwrap_or_default().into_bytes()
}
