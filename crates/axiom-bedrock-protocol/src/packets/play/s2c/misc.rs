use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_TEXT:             u8 = 0x09;
pub const ID_SET_TIME:         u8 = 0x1C;
pub const ID_SET_DIFFICULTY:   u8 = 0x3C;
pub const ID_SET_COMMANDS:     u8 = 0x4C;
pub const ID_LEVEL_EVENT:      u8 = 0x19;
pub const ID_SOUND_EVENT:      u8 = 0x7B;
pub const ID_BOSS_EVENT:       u8 = 0x4A;
pub const ID_TITLE:            u8 = 0x58;
pub const ID_TOAST:            u8 = 0xBB;

#[derive(Debug, Clone)]
pub struct SetTime {
    pub a: i32,
}

#[derive(Debug, Clone)]
pub struct SetDifficulty {
    pub a: u32,
}

#[derive(Debug, Clone)]
pub struct LevelEvent {
    pub a: i32,
    pub b: f32, pub c: f32, pub d: f32,
    pub e: i32,
}

#[derive(Debug, Clone)]
pub struct SoundEvent {
    pub a: String,
    pub b: f32, pub c: f32, pub d: f32,
    pub e: i32,
    pub f: i32,
    pub g: bool,
    pub h: bool,
}

#[derive(Debug, Clone)]
pub struct Title {
    pub a: i32,
    pub b: String,
    pub c: i32,
    pub d: i32,
    pub e: i32,
    pub f: String,
    pub g: String,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub a: String,
    pub b: String,
}

impl SetTime {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_SET_TIME);
        cx.varinti(self.a);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self { a: cx.varinti()? })
    }
}

impl SetDifficulty {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_SET_DIFFICULTY);
        cx.varint(self.a);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self { a: cx.varint()? })
    }
}

impl LevelEvent {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_LEVEL_EVENT);
        cx.varinti(self.a);
        cx.f32(self.b); cx.f32(self.c); cx.f32(self.d);
        cx.varinti(self.e);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.varinti()?,
            b: cx.f32()?, c: cx.f32()?, d: cx.f32()?,
            e: cx.varinti()?,
        })
    }
}

impl SoundEvent {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_SOUND_EVENT);
        cx.str(&self.a);
        cx.f32(self.b); cx.f32(self.c); cx.f32(self.d);
        cx.varinti(self.e);
        cx.varinti(self.f);
        cx.bool(self.g);
        cx.bool(self.h);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.str()?,
            b: cx.f32()?, c: cx.f32()?, d: cx.f32()?,
            e: cx.varinti()?,
            f: cx.varinti()?,
            g: cx.bool()?,
            h: cx.bool()?,
        })
    }
}

impl Title {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_TITLE);
        cx.varinti(self.a);
        cx.str(&self.b);
        cx.varinti(self.c);
        cx.varinti(self.d);
        cx.varinti(self.e);
        cx.str(&self.f);
        cx.str(&self.g);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.varinti()?,
            b: cx.str()?,
            c: cx.varinti()?,
            d: cx.varinti()?,
            e: cx.varinti()?,
            f: cx.str()?,
            g: cx.str()?,
        })
    }
}

impl Toast {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_TOAST);
        cx.str(&self.a);
        cx.str(&self.b);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self { a: cx.str()?, b: cx.str()? })
    }
}
