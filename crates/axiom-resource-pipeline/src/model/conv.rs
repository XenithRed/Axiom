use std::collections::HashMap;
use super::{
    bedrock::BedrockGeo,
    geo::{elements_to_bones, make_desc},
    java::JavaModel,
    resolve::Resolver,
};
use crate::error::{Err, R};

pub struct ModelConv {
    a: Resolver,
}

impl ModelConv {
    pub fn new() -> Self { Self { a: Resolver::new() } }

    pub fn load_pack(&mut self, files: &HashMap<String, Vec<u8>>) -> usize {
        let mut loaded = 0;
        for (path, data) in files {
            let rel = path.replace('\\', "/");
            if !rel.contains("/models/") { continue; }
            if !rel.ends_with(".json") { continue; }
            let name = stem_from_path(&rel);
            if let Ok(s) = std::str::from_utf8(data) {
                if let Ok(m) = JavaModel::from_json(s) {
                    self.a.add(name, m);
                    loaded += 1;
                }
            }
        }
        loaded
    }

    pub fn convert_all(&self) -> HashMap<String, Vec<u8>> {
        let mut out = HashMap::new();
        for (name, _) in self.a.a.iter() {
            if let Ok(geo) = self.convert_one(name) {
                if let Ok(json) = geo.to_json() {
                    let dst = format!("models/entity/{name}.geo.json");
                    out.insert(dst, json.into_bytes());
                }
            }
        }
        out
    }

    pub fn convert_one(&self, name: &str) -> R<BedrockGeo> {
        let model = self.a.resolve(name)?;
        let bones = elements_to_bones(name, &model);
        let desc  = make_desc(name, 16, 16);
        Ok(BedrockGeo {
            format_version: "1.12.0".to_string(),
            geometry: vec![super::bedrock::GeoEntry { description: desc, bones }],
        })
    }

    pub fn model_count(&self) -> usize { self.a.len() }
}

impl Default for ModelConv {
    fn default() -> Self { Self::new() }
}

fn stem_from_path(rel: &str) -> String {
    let parts: Vec<&str> = rel.split('/').collect();
    let fname = parts.last().unwrap_or(&"");
    fname.trim_end_matches(".json").to_string()
}
