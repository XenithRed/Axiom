use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JavaModel {
    pub parent:   Option<String>,
    pub textures: Option<HashMap<String, String>>,
    pub elements: Option<Vec<Element>>,
    pub display:  Option<HashMap<String, DisplayTransform>>,
    pub ambientocclusion: Option<bool>,
    pub gui_light: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Element {
    pub from:     [f32; 3],
    pub to:       [f32; 3],
    pub rotation: Option<Rotation>,
    pub faces:    Option<HashMap<String, Face>>,
    pub shade:    Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rotation {
    pub origin:  [f32; 3],
    pub axis:    String,
    pub angle:   f32,
    pub rescale: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Face {
    pub uv:        Option<[f32; 4]>,
    pub texture:   String,
    pub cullface:  Option<String>,
    pub rotation:  Option<i32>,
    pub tintindex: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DisplayTransform {
    pub rotation:    Option<[f32; 3]>,
    pub translation: Option<[f32; 3]>,
    pub scale:       Option<[f32; 3]>,
}

impl JavaModel {
    pub fn from_json(raw: &str) -> crate::error::R<Self> {
        serde_json::from_str(raw).map_err(|e| crate::error::Err::Model(e.to_string()))
    }

    pub fn textures(&self) -> HashMap<String, String> {
        self.textures.clone().unwrap_or_default()
    }

    pub fn elements(&self) -> &[Element] {
        self.elements.as_deref().unwrap_or(&[])
    }
}
