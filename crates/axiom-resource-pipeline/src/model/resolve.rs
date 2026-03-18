use std::collections::HashMap;
use super::java::JavaModel;
use crate::error::{Err, R};

const MAX_DEPTH: usize = 16;

pub struct Resolver {
    a: HashMap<String, JavaModel>,
}

impl Resolver {
    pub fn new() -> Self { Self { a: HashMap::new() } }

    pub fn add(&mut self, name: String, model: JavaModel) {
        self.a.insert(normalise(&name), model);
    }

    pub fn resolve(&self, name: &str) -> R<JavaModel> {
        self.resolve_depth(name, 0)
    }

    fn resolve_depth(&self, name: &str, depth: usize) -> R<JavaModel> {
        if depth > MAX_DEPTH {
            return Err(Err::Model(format!("parent chain too deep for {name}")));
        }
        let key = normalise(name);
        let m = self.a.get(&key).ok_or_else(|| Err::Model(format!("model not found: {name}")))?;
        if let Some(ref parent_name) = m.parent {
            if parent_name.starts_with("block/builtin") || parent_name == "builtin/generated" {
                return Ok(m.clone());
            }
            let mut parent = self.resolve_depth(parent_name, depth + 1)?;
            merge_into(&mut parent, m);
            Ok(parent)
        } else {
            Ok(m.clone())
        }
    }

    pub fn len(&self) -> usize { self.a.len() }
    pub fn is_empty(&self) -> bool { self.a.is_empty() }
}

fn merge_into(base: &mut JavaModel, child: &JavaModel) {
    if let Some(ref t) = child.textures {
        let map = base.textures.get_or_insert_with(Default::default);
        for (k, v) in t { map.insert(k.clone(), v.clone()); }
    }
    if child.elements.is_some() {
        base.elements = child.elements.clone();
    }
    if child.display.is_some() {
        base.display = child.display.clone();
    }
    if child.ambientocclusion.is_some() {
        base.ambientocclusion = child.ambientocclusion;
    }
}

fn normalise(s: &str) -> String {
    s.trim_start_matches("minecraft:").to_lowercase()
}

impl Default for Resolver {
    fn default() -> Self { Self::new() }
}
