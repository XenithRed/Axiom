use std::collections::HashMap;
use super::{anim, atlas};
use crate::error::R;

pub struct TexConv;

impl TexConv {
    pub fn convert(files: &HashMap<String, Vec<u8>>) -> HashMap<String, Vec<u8>> {
        let mut out = HashMap::new();

        let remapped = atlas::remap_paths(files);
        out.extend(remapped.clone());

        let flipbook = anim::convert_animations(files);
        if !flipbook.is_empty() {
            if let Ok(json) = anim::flipbook_to_json(&flipbook) {
                out.insert("textures/flipbook_textures.json".to_string(), json);
            }
        }

        let terrain = atlas::terrain_texture_json(&remapped);
        out.insert("textures/terrain_texture.json".to_string(), terrain);

        let items = atlas::item_texture_json(&remapped);
        out.insert("textures/item_texture.json".to_string(), items);

        out
    }

    pub fn count_textures(files: &HashMap<String, Vec<u8>>) -> usize {
        files.keys()
            .filter(|k| {
                let l = k.to_lowercase();
                l.contains("textures/") && l.ends_with(".png")
            })
            .count()
    }
}
