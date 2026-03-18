use super::{event::Event, state::Agsm};
use crate::{
    error::R,
    world::Chunk,
};

pub fn apply(agsm: &mut Agsm, ev: Event) -> R<Vec<Event>> {
    let mut side_effects: Vec<Event> = Vec::new();

    match ev {
        Event::PlayerJoin { a: uuid, b: name, c: edition } => {
            tracing::debug!(%uuid, %name, ?edition, "player join");
        }

        Event::PlayerLeave { a: uuid } => {
            agsm.leave_player(uuid);
            tracing::debug!(%uuid, "player leave");
        }

        Event::PlayerMove { a: uuid, b: x, c: y, d: z, e: yaw, f: pitch, g: on_ground } => {
            if let Some(p) = agsm.player_mut(uuid) {
                p.d.x = x; p.d.y = y; p.d.z = z;
                p.d.yaw = yaw; p.d.pitch = pitch;
                p.d.on_ground = on_ground;
            }
            if let Some(eid) = agsm.d.get(&uuid).copied() {
                let _ = agsm.b.move_entity(eid, x, y, z);
                let _ = agsm.b.rotate(eid, yaw, pitch);
            }
        }

        Event::PlayerHealth { a: uuid, b: hp, c: food, d: sat } => {
            if let Some(p) = agsm.player_mut(uuid) {
                p.g.set_health(hp);
                p.g.set_food(food);
            }
            if hp <= 0.0 {
                side_effects.push(Event::PlayerRespawn { a: uuid });
            }
        }

        Event::PlayerGameMode { a: uuid, b: mode } => {
            if let Some(p) = agsm.player_mut(uuid) {
                p.h = mode;
                p.e = match mode {
                    crate::player::GameMode::Creative  => crate::player::Abilities::creative(),
                    crate::player::GameMode::Spectator => crate::player::Abilities::spectator(),
                    _ => crate::player::Abilities::survival(),
                };
            }
        }

        Event::EntitySpawn { a: eid, b: kind, c: x, d: y, e: z } => {
            let mut entity = crate::entity::Entity::new(eid, kind, x, y, z);
            agsm.b.insert(entity);
        }

        Event::EntityDespawn { a: eid } => {
            agsm.b.remove(eid);
        }

        Event::EntityMove { a: eid, b: x, c: y, d: z, e: yaw, f: pitch } => {
            let _ = agsm.b.move_entity(eid, x, y, z);
            let _ = agsm.b.rotate(eid, yaw, pitch);
        }

        Event::EntityMeta { a: eid, b: idx, c: val } => {
            let _ = agsm.b.set_meta(eid, idx, val);
        }

        Event::EntityVelocity { a: eid, b: vx, c: vy, d: vz } => {
            let _ = agsm.b.set_vel(eid, vx, vy, vz);
        }

        Event::BlockChange { a: pos, b: state } => {
            let _ = agsm.set_block(pos, state);
        }

        Event::ChunkLoad { a: cx, b: cz, .. } => {
            if !agsm.a.is_loaded(cx, cz) {
                agsm.a.load(Chunk::new(cx, cz));
            }
        }

        Event::ChunkUnload { a: cx, b: cz } => {
            agsm.a.unload(cx, cz);
        }

        Event::TimeChange { a: t } => {
            agsm.f = t;
        }

        Event::DifficultyChange { a: d } => {
            agsm.g = super::state::Difficulty::from_i32(d);
        }

        _ => {}
    }

    Ok(side_effects)
}

pub fn tick(agsm: &mut Agsm) -> Vec<Event> {
    agsm.advance_tick();
    Vec::new()
}
