use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_CLIENT_INFO:    i32 = 0x08;
pub const ID_KEEP_ALIVE:     i32 = 0x18;
pub const ID_PONG:           i32 = 0x26;
pub const ID_RESPAWN:        i32 = 0x09;
pub const ID_SWING_ARM:      i32 = 0x36;
pub const ID_PLAYER_COMMAND: i32 = 0x25;
pub const ID_PLUGIN_MSG:     i32 = 0x12;
pub const ID_STATUS_REQUEST: i32 = 0x00;
pub const ID_PING_REQUEST:   i32 = 0x01;

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub a: String,
    pub b: i8,
    pub c: i32,
    pub d: bool,
    pub e: u8,
    pub f: i32,
    pub g: bool,
    pub h: bool,
}

#[derive(Debug, Clone)]
pub struct KeepAlive { pub a: i64 }

#[derive(Debug, Clone)]
pub struct Pong { pub a: i32 }

#[derive(Debug, Clone)]
pub struct SwingArm { pub a: i32 }

#[derive(Debug, Clone)]
pub struct PlayerCommand { pub a: i32, pub b: i32, pub c: i32 }

#[derive(Debug, Clone)]
pub struct PluginMsg { pub a: String, pub b: Bytes }

impl ClientInfo {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CLIENT_INFO);
        cx.str_max(&self.a, 16);
        cx.i8(self.b); cx.var32(self.c);
        cx.bool(self.d); cx.u8(self.e); cx.var32(self.f);
        cx.bool(self.g); cx.bool(self.h);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self {
            a: cx.str_max(16)?,
            b: cx.i8()?, c: cx.var32()?,
            d: cx.bool()?, e: cx.u8()?, f: cx.var32()?,
            g: cx.bool()?, h: cx.bool()?,
        })
    }
}

impl KeepAlive {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_KEEP_ALIVE); cx.i64(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.i64()? })
    }
}

impl Pong {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_PONG); cx.i32(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.i32()? })
    }
}

impl SwingArm {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_SWING_ARM); cx.var32(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.var32()? })
    }
}

impl PlayerCommand {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_PLAYER_COMMAND);
        cx.var32(self.a); cx.var32(self.b); cx.var32(self.c); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.var32()?, b: cx.var32()?, c: cx.var32()? })
    }
}

impl PluginMsg {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_PLUGIN_MSG);
        cx.str(&self.a); cx.bytes(&self.b); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.str()?, b: cx.rest() })
    }
}
