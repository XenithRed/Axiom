pub mod error;
pub mod io;
pub mod model;
pub mod shader;
pub mod sound;
pub mod tex;

pub use error::{Err, R};
pub use model::ModelConv;
pub use tex::TexConv;

use std::collections::HashMap;
use sha2::{Digest, Sha256};

#[derive(Debug, Default)]
pub struct Report {
    pub models:   u32,
    pub textures: u32,
    pub sounds:   bool,
    pub warnings: Vec<String>,
}

pub struct Pipeline {
    a: ModelConv,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { a: ModelConv::new() }
    }

    pub fn convert(&mut self, src: &io::Pack) -> R<(HashMap<String, Vec<u8>>, Report)> {
        let mut out  = HashMap::new();
        let mut rep  = Report::default();

        let all = src.all_files();
        let mut files: HashMap<String, Vec<u8>> = HashMap::new();
        for f in &all {
            if let Some(d) = src.read(f) { files.insert(f.clone(), d); }
        }

        let loaded = self.a.load_pack(&files);
        let model_files = self.a.convert_all();
        rep.models = model_files.len() as u32;
        out.extend(model_files);

        let tex_files = TexConv::convert(&files);
        rep.textures = TexConv::count_textures(&tex_files) as u32;
        out.extend(tex_files);

        let sounds_key = "assets/minecraft/sounds.json";
        if let Some(raw) = files.get(sounds_key) {
            match sound::convert_sounds(raw) {
                Ok(json) => {
                    out.insert("sounds/sound_definitions.json".to_string(), json);
                    rep.sounds = true;
                }
                Err(e) => rep.warnings.push(format!("sounds: {e}")),
            }
        }

        for (path, data) in &files {
            if path.ends_with(".ogg") {
                let dst = path.replace("assets/minecraft/sounds/", "sounds/");
                out.insert(dst, data.clone());
            }
        }

        out.insert("manifest.json".to_string(), manifest_json());
        out.insert("pack_icon.png".to_string(), placeholder_icon());

        Ok((out, rep))
    }
}

impl Default for Pipeline {
    fn default() -> Self { Self::new() }
}

fn manifest_json() -> Vec<u8> {
    let uid_h = new_uuid(0);
    let uid_m = new_uuid(1);
    let json = serde_json::json!({
        "format_version": 2,
        "header": {
            "description": "Converted by axiom-resource-pipeline",
            "name":        "Axiom Converted Pack",
            "uuid":        uid_h,
            "version":     [1, 0, 0],
            "min_engine_version": [1, 20, 0],
        },
        "modules": [{
            "description": "Resource pack",
            "type":        "resources",
            "uuid":        uid_m,
            "version":     [1, 0, 0],
        }]
    });
    serde_json::to_string_pretty(&json).unwrap_or_default().into_bytes()
}

fn new_uuid(salt: u8) -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut h = Sha256::digest([ts.to_le_bytes().as_ref(), &[salt]].concat());
    let b = h.as_slice();
    format!("{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0],b[1],b[2],b[3], b[4],b[5], b[6],b[7], b[8],b[9], b[10],b[11],b[12],b[13],b[14],b[15])
}

fn placeholder_icon() -> Vec<u8> {
    vec![
        0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A,
        0x00,0x00,0x00,0x0D,0x49,0x48,0x44,0x52,
        0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,
        0x08,0x02,0x00,0x00,0x00,0x90,0x77,0x53,
        0xDE,0x00,0x00,0x00,0x0C,0x49,0x44,0x41,
        0x54,0x08,0xD7,0x63,0xF8,0xCF,0xC0,0x00,
        0x00,0x00,0x02,0x00,0x01,0xE2,0x21,0xBC,
        0x33,0x00,0x00,0x00,0x00,0x49,0x45,0x4E,
        0x44,0xAE,0x42,0x60,0x82,
    ]
}
