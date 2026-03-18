pub fn bedrock_materials_json() -> Vec<u8> {
    let json = serde_json::json!({
        "materials": {
            "version": "1.0.0",
            "entity": { "+states": ["EnableStencilTest", "StencilWrite"] },
            "entity_alphatest": { "+defines": ["ALPHA_TEST"] },
        }
    });
    serde_json::to_string_pretty(&json).unwrap_or_default().into_bytes()
}

pub fn render_controllers_json(entity_name: &str) -> Vec<u8> {
    let json = serde_json::json!({
        "render_controllers": {
            format!("controller.render.{entity_name}"): {
                "geometry": format!("Geometry.{entity_name}"),
                "materials": [{ "*": "Material.default" }],
                "textures":  [format!("Texture.{entity_name}")],
            }
        }
    });
    serde_json::to_string_pretty(&json).unwrap_or_default().into_bytes()
}
