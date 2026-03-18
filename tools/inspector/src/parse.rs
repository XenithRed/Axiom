use bytes::Bytes;
use crate::cap::{Dir, Frame};

#[derive(Debug, Clone)]
pub struct ParsedPkt {
    pub a: u64,
    pub b: Dir,
    pub c: i32,
    pub d: String,
    pub e: usize,
    pub f: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub a: &'static str,
    pub b: String,
}

impl ParsedPkt {
    pub fn id_hex(&self) -> String {
        format!("0x{:02X}", self.c)
    }
}

pub fn parse(frame: &Frame) -> ParsedPkt {
    let raw = &frame.d;
    let id = peek_var32(raw).unwrap_or(-1);
    let name = id_name(frame.c, id);
    let fields = decode_fields(frame.c, id, raw);

    ParsedPkt {
        a: frame.a,
        b: frame.c,
        c: id,
        d: name.to_string(),
        e: raw.len(),
        f: fields,
    }
}

fn peek_var32(b: &[u8]) -> Option<i32> {
    let (mut n, mut s) = (0u32, 0u32);
    for &x in b.iter().take(5) {
        n |= ((x & 0x7F) as u32) << s;
        if x & 0x80 == 0 { return Some(n as i32); }
        s += 7;
    }
    None
}

fn id_name(dir: Dir, id: i32) -> &'static str {
    match (dir, id) {
        (Dir::C2S, 0x00) => "Handshake / LoginStart / ConfirmTP",
        (Dir::C2S, 0x01) => "EncResponse / AttackBlock",
        (Dir::C2S, 0x04) => "Command",
        (Dir::C2S, 0x06) => "Chat",
        (Dir::C2S, 0x08) => "ClientInfo",
        (Dir::C2S, 0x0C) => "ClickContainer",
        (Dir::C2S, 0x0D) => "CloseContainer",
        (Dir::C2S, 0x12) => "PluginMessage",
        (Dir::C2S, 0x13) => "Interact",
        (Dir::C2S, 0x18) => "KeepAlive",
        (Dir::C2S, 0x1A) => "MovePlayerPos",
        (Dir::C2S, 0x1B) => "MovePlayerRot",
        (Dir::C2S, 0x1C) => "MovePlayerFull",
        (Dir::C2S, 0x1D) => "MovePlayerOnGround",
        (Dir::C2S, 0x22) => "DropItem",
        (Dir::C2S, 0x24) => "PlayerAction",
        (Dir::C2S, 0x25) => "PlayerCommand",
        (Dir::C2S, 0x26) => "Pong",
        (Dir::C2S, 0x36) => "UseItemOn",
        (Dir::C2S, 0x37) => "UseItem",
        (Dir::C2S, 0x3B) => "CreativeInventory",
        (Dir::S2C, 0x01) => "SpawnEntity",
        (Dir::S2C, 0x09) => "BlockUpdate",
        (Dir::S2C, 0x0B) => "MultiBlockChange",
        (Dir::S2C, 0x12) => "CloseScreen",
        (Dir::S2C, 0x13) => "ContainerContent",
        (Dir::S2C, 0x15) => "ContainerSlot",
        (Dir::S2C, 0x18) => "PluginMessage",
        (Dir::S2C, 0x1C) => "Disconnect",
        (Dir::S2C, 0x1F) => "UnloadChunk",
        (Dir::S2C, 0x22) => "GameEvent",
        (Dir::S2C, 0x26) => "KeepAlive",
        (Dir::S2C, 0x27) => "ChunkDataAndLight",
        (Dir::S2C, 0x2B) => "Login",
        (Dir::S2C, 0x2C) => "MoveEntityPos",
        (Dir::S2C, 0x2D) => "MoveEntityFull",
        (Dir::S2C, 0x2E) => "MoveEntityRot",
        (Dir::S2C, 0x31) => "OpenScreen",
        (Dir::S2C, 0x37) => "Ping",
        (Dir::S2C, 0x38) => "PlayerAbilities",
        (Dir::S2C, 0x39) => "PlayerChat",
        (Dir::S2C, 0x3E) => "PlayerInfoUpdate",
        (Dir::S2C, 0x40) => "PlayerPosition",
        (Dir::S2C, 0x42) => "RemoveEntities",
        (Dir::S2C, 0x47) => "Respawn",
        (Dir::S2C, 0x54) => "SetCenterChunk",
        (Dir::S2C, 0x58) => "SetExperience",
        (Dir::S2C, 0x59) => "SetHealth",
        (Dir::S2C, 0x5A) => "SetTime",
        (Dir::S2C, 0x5C) => "SetEntityVelocity",
        (Dir::S2C, 0x61) => "SubtitleText",
        (Dir::S2C, 0x62) => "TitleText",
        (Dir::S2C, 0x63) => "TitleTimes",
        (Dir::S2C, 0x64) => "SoundEffect",
        (Dir::S2C, 0x68) => "TabListHeader",
        (Dir::S2C, 0x6C) => "SystemChat",
        (Dir::S2C, 0x76) => "EntityEffect",
        _ => "Unknown",
    }
}

fn decode_fields(dir: Dir, id: i32, raw: &[u8]) -> Vec<Field> {
    let mut b = raw;
    skip_var32(&mut b);

    match (dir, id) {
        (Dir::S2C, 0x27) => {
            let x = read_var32i(&mut b).unwrap_or(0);
            let z = read_var32i(&mut b).unwrap_or(0);
            vec![
                Field { a: "chunk_x", b: x.to_string() },
                Field { a: "chunk_z", b: z.to_string() },
                Field { a: "payload_bytes", b: raw.len().to_string() },
            ]
        }
        (Dir::S2C, 0x59) => {
            let hp  = read_f32(&mut b).unwrap_or(0.0);
            let food = read_var32i(&mut b).unwrap_or(0);
            let sat  = read_f32(&mut b).unwrap_or(0.0);
            vec![
                Field { a: "health", b: format!("{hp:.1}") },
                Field { a: "food",   b: food.to_string() },
                Field { a: "saturation", b: format!("{sat:.2}") },
            ]
        }
        (Dir::S2C, 0x6C) => {
            let msg = read_str(&mut b).unwrap_or_default();
            let overlay = read_bool(&mut b).unwrap_or(false);
            vec![
                Field { a: "message", b: msg },
                Field { a: "overlay", b: overlay.to_string() },
            ]
        }
        (Dir::C2S, 0x1C) => {
            let x = read_f64(&mut b).unwrap_or(0.0);
            let y = read_f64(&mut b).unwrap_or(0.0);
            let z = read_f64(&mut b).unwrap_or(0.0);
            let yaw   = read_f32(&mut b).unwrap_or(0.0);
            let pitch = read_f32(&mut b).unwrap_or(0.0);
            vec![
                Field { a: "x", b: format!("{x:.3}") },
                Field { a: "y", b: format!("{y:.3}") },
                Field { a: "z", b: format!("{z:.3}") },
                Field { a: "yaw",   b: format!("{yaw:.1}") },
                Field { a: "pitch", b: format!("{pitch:.1}") },
            ]
        }
        (Dir::C2S, 0x06) => {
            let msg = read_str(&mut b).unwrap_or_default();
            vec![ Field { a: "message", b: msg } ]
        }
        (Dir::S2C, 0x26) | (Dir::C2S, 0x18) => {
            let id = read_i64(&mut b).unwrap_or(0);
            vec![ Field { a: "keepalive_id", b: id.to_string() } ]
        }
        _ => vec![ Field { a: "bytes", b: raw.len().to_string() } ],
    }
}

fn skip_var32(b: &mut &[u8]) {
    for i in 0..5 {
        if b.is_empty() { return; }
        let x = b[0]; *b = &b[1..];
        if x & 0x80 == 0 { return; }
    }
}

fn read_var32i(b: &mut &[u8]) -> Option<i32> {
    let (mut n, mut s) = (0u32, 0u32);
    loop {
        if b.is_empty() { return None; }
        let x = b[0]; *b = &b[1..];
        n |= ((x & 0x7F) as u32) << s;
        if x & 0x80 == 0 { return Some(n as i32); }
        s += 7;
        if s >= 35 { return None; }
    }
}

fn read_f32(b: &mut &[u8]) -> Option<f32> {
    if b.len() < 4 { return None; }
    let v = f32::from_bits(u32::from_be_bytes([b[0],b[1],b[2],b[3]]));
    *b = &b[4..]; Some(v)
}

fn read_f64(b: &mut &[u8]) -> Option<f64> {
    if b.len() < 8 { return None; }
    let v = f64::from_bits(u64::from_be_bytes(b[..8].try_into().unwrap()));
    *b = &b[8..]; Some(v)
}

fn read_i64(b: &mut &[u8]) -> Option<i64> {
    if b.len() < 8 { return None; }
    let v = i64::from_be_bytes(b[..8].try_into().unwrap());
    *b = &b[8..]; Some(v)
}

fn read_bool(b: &mut &[u8]) -> Option<bool> {
    if b.is_empty() { return None; }
    let v = b[0] != 0; *b = &b[1..]; Some(v)
}

fn read_str(b: &mut &[u8]) -> Option<String> {
    let n = read_var32i(b)? as usize;
    if b.len() < n { return None; }
    let s = String::from_utf8_lossy(&b[..n]).into_owned();
    *b = &b[n..]; Some(s)
}
