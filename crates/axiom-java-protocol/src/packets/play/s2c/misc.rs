use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_KEEP_ALIVE:       i32 = 0x26;
pub const ID_PING:             i32 = 0x37;
pub const ID_PLUGIN_MSG:       i32 = 0x18;
pub const ID_SOUND:            i32 = 0x64;
pub const ID_NAMED_SOUND:      i32 = 0x63;
pub const ID_STOP_SOUND:       i32 = 0x6E;
pub const ID_TITLE_TEXT:       i32 = 0x62;
pub const ID_SUBTITLE_TEXT:    i32 = 0x61;
pub const ID_ACTION_BAR:       i32 = 0x4A;
pub const ID_CLEAR_TITLES:     i32 = 0x4D;
pub const ID_TITLE_TIMES:      i32 = 0x63;
pub const ID_PLAYER_XP:        i32 = 0x58;
pub const ID_SET_BORDER:       i32 = 0x50;
pub const ID_AWARD_STATS:      i32 = 0x08;
pub const ID_BOSS_BAR:         i32 = 0x0A;
pub const ID_TABLIST_HEADER:   i32 = 0x68;

#[derive(Debug, Clone)]
pub struct KeepAlive { pub a: i64 }

#[derive(Debug, Clone)]
pub struct Ping { pub a: i32 }

#[derive(Debug, Clone)]
pub struct PluginMsg { pub a: String, pub b: Bytes }

#[derive(Debug, Clone)]
pub struct Sound {
    pub a: i32,
    pub b: i32,
    pub c: i32, pub d: i32, pub e: i32,
    pub f: f32, pub g: f32,
    pub h: i64,
}

#[derive(Debug, Clone)]
pub struct TitleText { pub a: String }

#[derive(Debug, Clone)]
pub struct SubtitleText { pub a: String }

#[derive(Debug, Clone)]
pub struct ActionBar { pub a: String }

#[derive(Debug, Clone)]
pub struct ClearTitles { pub a: bool }

#[derive(Debug, Clone)]
pub struct TitleTimes { pub a: i32, pub b: i32, pub c: i32 }

#[derive(Debug, Clone)]
pub struct TablistHeader { pub a: String, pub b: String }

impl KeepAlive {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_KEEP_ALIVE); cx.i64(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.i64()? })
    }
}

impl Ping {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_PING); cx.i32(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.i32()? })
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

impl TitleText {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_TITLE_TEXT); cx.str(&self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.str()? })
    }
}

impl SubtitleText {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_SUBTITLE_TEXT); cx.str(&self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.str()? })
    }
}

impl ActionBar {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_ACTION_BAR); cx.str(&self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.str()? })
    }
}

impl ClearTitles {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CLEAR_TITLES); cx.bool(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.bool()? })
    }
}

impl TitleTimes {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_TITLE_TIMES);
        cx.i32(self.a); cx.i32(self.b); cx.i32(self.c); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.i32()?, b: cx.i32()?, c: cx.i32()? })
    }
}

impl TablistHeader {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_TABLIST_HEADER);
        cx.str(&self.a); cx.str(&self.b); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.str()?, b: cx.str()? })
    }
}
