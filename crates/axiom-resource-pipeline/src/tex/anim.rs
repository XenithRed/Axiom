use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::R;

#[derive(Debug, Deserialize)]
struct McMeta {
    animation: Option<AnimMeta>,
}

#[derive(Debug, Deserialize)]
struct AnimMeta {
    frametime: Option<u32>,
    frames:    Option<Vec<serde_json::Value>>,
    interpolate: Option<bool>,
    width:  Option<u32>,
    height: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct FlipbookEntry {
    pub flipbook_texture: String,
    pub atlas_tile:       String,
    pub ticks_per_frame:  u32,
    pub blend_frames:     bool,
}

pub fn convert_animations(
    files: &HashMap<String, Vec<u8>>,
) -> Vec<FlipbookEntry> {
    let mut out = Vec::new();

    for (path, data) in files {
        if !path.ends_with(".png.mcmeta") { continue; }
        let tex_path = path.trim_end_matches(".mcmeta");
        let name = tex_path
            .split('/')
            .last()
            .unwrap_or("")
            .trim_end_matches(".png")
            .to_string();

        if let Ok(s) = std::str::from_utf8(data) {
            if let Ok(meta) = serde_json::from_str::<McMeta>(s) {
                if let Some(anim) = meta.animation {
                    out.push(FlipbookEntry {
                        flipbook_texture: format!("textures/blocks/{name}"),
                        atlas_tile:       name.clone(),
                        ticks_per_frame:  anim.frametime.unwrap_or(1),
                        blend_frames:     anim.interpolate.unwrap_or(false),
                    });
                }
            }
        }
    }

    out
}

pub fn flipbook_to_json(entries: &[FlipbookEntry]) -> R<Vec<u8>> {
    let json = serde_json::to_string_pretty(entries)?;
    Ok(json.into_bytes())
}
