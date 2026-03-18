use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use crate::report::{Report, Issue, Severity};

#[derive(Debug, Clone, Deserialize)]
pub struct JavaModel {
    pub parent:   Option<String>,
    pub textures: Option<HashMap<String, String>>,
    pub elements: Option<Vec<Element>>,
    pub display:  Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Element {
    pub from:     [f32; 3],
    pub to:       [f32; 3],
    pub rotation: Option<ElemRotation>,
    pub faces:    Option<HashMap<String, Face>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ElemRotation {
    pub origin: [f32; 3],
    pub axis:   String,
    pub angle:  f32,
    pub rescale: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Face {
    pub uv:      Option<[f32; 4]>,
    pub texture: String,
    pub cullface: Option<String>,
    pub rotation: Option<i32>,
    pub tintindex: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BedrockGeo {
    pub format_version: String,
    #[serde(rename = "minecraft:geometry")]
    pub geometry: Vec<GeoEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeoEntry {
    pub description: GeoDesc,
    pub bones: Vec<Bone>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeoDesc {
    pub identifier:           String,
    pub texture_width:        u32,
    pub texture_height:       u32,
    pub visible_bounds_width: f32,
    pub visible_bounds_height: f32,
    pub visible_bounds_offset: [f32; 3],
}

#[derive(Debug, Clone, Serialize)]
pub struct Bone {
    pub name:   String,
    pub pivot:  [f32; 3],
    pub rotation: Option<[f32; 3]>,
    pub cubes:  Vec<Cube>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Cube {
    pub origin: [f32; 3],
    pub size:   [f32; 3],
    pub uv:     serde_json::Value,
    pub inflate: Option<f32>,
}

pub struct ConvCtx<'a> {
    pub a: &'a Path,
    pub b: &'a Path,
    pub c: &'a mut Report,
}

pub fn convert_pack(src: &Path, dst: &Path) -> crate::Result<Report> {
    let mut report = Report::new();
    let mut ctx = ConvCtx { a: src, b: dst, c: &mut report };

    fs::create_dir_all(dst)?;

    convert_textures(&mut ctx)?;
    convert_models(&mut ctx)?;
    convert_sounds(&mut ctx)?;
    write_manifest(&mut ctx)?;

    Ok(report)
}

fn convert_textures(ctx: &mut ConvCtx) -> crate::Result<()> {
    let src_tex = ctx.a.join("assets").join("minecraft").join("textures");
    let dst_tex = ctx.b.join("textures");
    if !src_tex.exists() { return Ok(()); }

    for entry in walkdir::WalkDir::new(&src_tex)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "png").unwrap_or(false))
    {
        let rel = entry.path().strip_prefix(&src_tex).unwrap();
        let dst_file = dst_tex.join(remap_texture_path(rel));
        if let Some(p) = dst_file.parent() { fs::create_dir_all(p)?; }

        match process_texture(entry.path(), &dst_file) {
            Ok(_) => {}
            Err(e) => {
                ctx.c.add(Issue {
                    a: Severity::Warn,
                    b: format!("texture {}: {e}", rel.display()),
                });
            }
        }
    }
    Ok(())
}

fn process_texture(src: &Path, dst: &Path) -> crate::Result<()> {
    let data = fs::read(src)?;
    fs::write(dst, data)?;
    Ok(())
}

fn remap_texture_path(p: &Path) -> PathBuf {
    let s = p.to_string_lossy().replace('\\', "/");
    let s = s
        .replace("block/", "blocks/")
        .replace("item/",  "items/")
        .replace("entity/", "entity/");
    PathBuf::from(s)
}

fn convert_models(ctx: &mut ConvCtx) -> crate::Result<()> {
    let src_models = ctx.a.join("assets").join("minecraft").join("models").join("block");
    let dst_geo    = ctx.b.join("models").join("entity");
    if !src_models.exists() { return Ok(()); }
    fs::create_dir_all(&dst_geo)?;

    for entry in walkdir::WalkDir::new(&src_models)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
    {
        let stem = entry.path().file_stem().unwrap().to_string_lossy().to_string();
        match convert_model(entry.path(), &stem) {
            Ok(geo) => {
                let dst = dst_geo.join(format!("{stem}.geo.json"));
                let json = serde_json::to_string_pretty(&geo)?;
                fs::write(&dst, json)?;
                ctx.c.converted += 1;
            }
            Err(e) => {
                ctx.c.add(Issue {
                    a: Severity::Warn,
                    b: format!("model {stem}: {e}"),
                });
            }
        }
    }
    Ok(())
}

fn convert_model(path: &Path, name: &str) -> crate::Result<BedrockGeo> {
    let raw = fs::read_to_string(path)?;
    let model: JavaModel = serde_json::from_str(&raw)?;

    let elements = model.elements.unwrap_or_default();
    let mut bones: Vec<Bone> = Vec::new();

    for (i, elem) in elements.iter().enumerate() {
        let origin = [
            elem.from[0] - 8.0,
            elem.from[1],
            elem.from[2] - 8.0,
        ];
        let size = [
            elem.to[0] - elem.from[0],
            elem.to[1] - elem.from[1],
            elem.to[2] - elem.from[2],
        ];
        let pivot = [
            (elem.from[0] + elem.to[0]) / 2.0 - 8.0,
            (elem.from[1] + elem.to[1]) / 2.0,
            (elem.from[2] + elem.to[2]) / 2.0 - 8.0,
        ];
        let rotation = elem.rotation.as_ref().map(|r| {
            let v = r.angle;
            match r.axis.as_str() {
                "x" => [v, 0.0, 0.0],
                "y" => [0.0, v, 0.0],
                "z" => [0.0, 0.0, v],
                _   => [0.0, 0.0, 0.0],
            }
        });

        let cube = Cube {
            origin,
            size,
            uv: serde_json::json!([0, 0]),
            inflate: None,
        };

        bones.push(Bone {
            name: format!("{name}_elem_{i}"),
            pivot,
            rotation,
            cubes: vec![cube],
        });
    }

    Ok(BedrockGeo {
        format_version: "1.12.0".to_string(),
        geometry: vec![GeoEntry {
            description: GeoDesc {
                identifier:            format!("geometry.{name}"),
                texture_width:         16,
                texture_height:        16,
                visible_bounds_width:  2.0,
                visible_bounds_height: 2.5,
                visible_bounds_offset: [0.0, 0.75, 0.0],
            },
            bones,
        }],
    })
}

fn convert_sounds(ctx: &mut ConvCtx) -> crate::Result<()> {
    let src = ctx.a.join("assets").join("minecraft").join("sounds.json");
    if !src.exists() { return Ok(()); }

    let raw = fs::read_to_string(&src)?;
    let java: serde_json::Value = serde_json::from_str(&raw)?;

    let mut bedrock: HashMap<String, serde_json::Value> = HashMap::new();

    if let Some(obj) = java.as_object() {
        for (k, v) in obj {
            let sounds: Vec<String> = v["sounds"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|s| {
                    s.as_str().map(|s| s.to_string())
                        .or_else(|| s["name"].as_str().map(|s| s.to_string()))
                }).collect())
                .unwrap_or_default();

            if !sounds.is_empty() {
                bedrock.insert(k.clone(), serde_json::json!({
                    "sounds": sounds.iter().map(|s| {
                        serde_json::json!({
                            "name": format!("sounds/{s}"),
                            "stream": false
                        })
                    }).collect::<Vec<_>>()
                }));
            }
        }
    }

    let dst = ctx.b.join("sounds");
    fs::create_dir_all(&dst)?;
    let json = serde_json::to_string_pretty(&serde_json::json!({ "sound_definitions": bedrock }))?;
    fs::write(dst.join("sound_definitions.json"), json)?;

    Ok(())
}

fn write_manifest(ctx: &mut ConvCtx) -> crate::Result<()> {
    let uuid_hdr  = new_uuid();
    let uuid_mods = new_uuid();

    let manifest = serde_json::json!({
        "format_version": 2,
        "header": {
            "description": "Converted by axiom-rpconv",
            "name": "Axiom Converted Pack",
            "uuid": uuid_hdr,
            "version": [1, 0, 0],
            "min_engine_version": [1, 16, 0]
        },
        "modules": [{
            "description": "Resource pack",
            "type": "resources",
            "uuid": uuid_mods,
            "version": [1, 0, 0]
        }]
    });

    let json = serde_json::to_string_pretty(&manifest)?;
    fs::write(ctx.b.join("manifest.json"), json)?;
    Ok(())
}

fn new_uuid() -> String {
    use sha2::{Digest, Sha256};
    use std::time::SystemTime;
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let hash = Sha256::digest(ts.to_le_bytes());
    let h = hex::encode(&hash[..16]);
    format!("{}-{}-{}-{}-{}",
        &h[0..8], &h[8..12], &h[12..16], &h[16..20], &h[20..32])
}
