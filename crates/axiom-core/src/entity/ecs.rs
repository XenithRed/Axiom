use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use super::{id::Eid, kind::Kind, meta::Meta};
use crate::error::{Err, R};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub a: Eid,
    pub b: Kind,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f32,
    pub g: f32,
    pub h: f64,
    pub i: f64,
    pub j: f64,
    pub k: Meta,
    pub l: bool,
}

impl Entity {
    pub fn new(id: Eid, kind: Kind, x: f64, y: f64, z: f64) -> Self {
        Self {
            a: id, b: kind,
            c: x, d: y, e: z,
            f: 0.0, g: 0.0,
            h: 0.0, i: 0.0, j: 0.0,
            k: Meta::new(),
            l: false,
        }
    }

    pub fn pos(&self) -> (f64, f64, f64) { (self.c, self.d, self.e) }
    pub fn yaw(&self) -> f32 { self.f }
    pub fn pitch(&self) -> f32 { self.g }
    pub fn vel(&self) -> (f64, f64, f64) { (self.h, self.i, self.j) }
    pub fn on_ground(&self) -> bool { self.l }
}

#[derive(Debug, Default)]
pub struct Ecs {
    a: AHashMap<Eid, Entity>,
}

impl Ecs {
    pub fn new() -> Self { Self::default() }

    pub fn spawn(&mut self, kind: Kind, x: f64, y: f64, z: f64) -> Eid {
        let id = Eid::next();
        self.a.insert(id, Entity::new(id, kind, x, y, z));
        id
    }

    pub fn insert(&mut self, e: Entity) {
        self.a.insert(e.a, e);
    }

    pub fn remove(&mut self, id: Eid) -> Option<Entity> {
        self.a.remove(&id)
    }

    pub fn get(&self, id: Eid) -> R<&Entity> {
        self.a.get(&id).ok_or(Err::NoEntity(id.raw()))
    }

    pub fn get_mut(&mut self, id: Eid) -> R<&mut Entity> {
        self.a.get_mut(&id).ok_or(Err::NoEntity(id.raw()))
    }

    pub fn move_entity(&mut self, id: Eid, x: f64, y: f64, z: f64) -> R<()> {
        let e = self.get_mut(id)?;
        e.c = x; e.d = y; e.e = z;
        Ok(())
    }

    pub fn rotate(&mut self, id: Eid, yaw: f32, pitch: f32) -> R<()> {
        let e = self.get_mut(id)?;
        e.f = yaw; e.g = pitch;
        Ok(())
    }

    pub fn set_vel(&mut self, id: Eid, vx: f64, vy: f64, vz: f64) -> R<()> {
        let e = self.get_mut(id)?;
        e.h = vx; e.i = vy; e.j = vz;
        Ok(())
    }

    pub fn set_meta(&mut self, id: Eid, idx: u8, val: super::meta::MetaVal) -> R<()> {
        self.get_mut(id)?.k.set(idx, val);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.a.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.a.values_mut()
    }

    pub fn count(&self) -> usize { self.a.len() }

    pub fn contains(&self, id: Eid) -> bool { self.a.contains_key(&id) }

    pub fn nearby(&self, x: f64, y: f64, z: f64, radius: f64) -> Vec<&Entity> {
        let r2 = radius * radius;
        self.a.values().filter(|e| {
            let dx = e.c - x;
            let dy = e.d - y;
            let dz = e.e - z;
            dx*dx + dy*dy + dz*dz <= r2
        }).collect()
    }
}
