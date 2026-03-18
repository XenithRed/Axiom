use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedrockGeo {
    pub format_version: String,
    #[serde(rename = "minecraft:geometry")]
    pub geometry: Vec<GeoEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoEntry {
    pub description: GeoDesc,
    pub bones: Vec<Bone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoDesc {
    pub identifier:            String,
    pub texture_width:         u32,
    pub texture_height:        u32,
    pub visible_bounds_width:  f32,
    pub visible_bounds_height: f32,
    pub visible_bounds_offset: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bone {
    pub name:     String,
    pub pivot:    [f32; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<[f32; 3]>,
    pub cubes:    Vec<Cube>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cube {
    pub origin:  [f32; 3],
    pub size:    [f32; 3],
    pub uv:      serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inflate: Option<f32>,
}

impl BedrockGeo {
    pub fn to_json(&self) -> crate::error::R<String> {
        serde_json::to_string_pretty(self).map_err(|e| crate::error::Err::Model(e.to_string()))
    }
}
