use super::{bedrock::{Bone, Cube, GeoDesc, GeoEntry}, java::JavaModel};

pub fn elements_to_bones(name: &str, model: &JavaModel) -> Vec<Bone> {
    model.elements().iter().enumerate().map(|(i, elem)| {
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
            let a = r.angle;
            match r.axis.as_str() {
                "x" => [-a, 0.0, 0.0],
                "y" => [0.0, -a, 0.0],
                "z" => [0.0, 0.0, a],
                _   => [0.0, 0.0, 0.0],
            }
        });
        let uv = face_uv(elem);
        Bone {
            name:     format!("{name}_b{i}"),
            pivot,
            rotation,
            cubes: vec![Cube { origin, size, uv, inflate: None }],
        }
    }).collect()
}

pub fn make_desc(name: &str, tw: u32, th: u32) -> GeoDesc {
    GeoDesc {
        identifier:            format!("geometry.{name}"),
        texture_width:         tw,
        texture_height:        th,
        visible_bounds_width:  2.0,
        visible_bounds_height: 2.5,
        visible_bounds_offset: [0.0, 0.75, 0.0],
    }
}

fn face_uv(elem: &super::java::Element) -> serde_json::Value {
    if let Some(ref faces) = elem.faces {
        let mut obj = serde_json::Map::new();
        for (face_name, face) in faces {
            let uv = face.uv.unwrap_or([0.0, 0.0, 16.0, 16.0]);
            let rot = face.rotation.unwrap_or(0);
            let bedrock_face = remap_face(face_name);
            obj.insert(bedrock_face, serde_json::json!({
                "uv":      [uv[0], uv[1]],
                "uv_size": [uv[2] - uv[0], uv[3] - uv[1]],
                "rotation": rot,
            }));
        }
        serde_json::Value::Object(obj)
    } else {
        serde_json::json!([0, 0])
    }
}

fn remap_face(java: &str) -> String {
    match java {
        "up"    => "up",
        "down"  => "down",
        "north" => "north",
        "south" => "south",
        "east"  => "east",
        "west"  => "west",
        other   => other,
    }.to_string()
}
