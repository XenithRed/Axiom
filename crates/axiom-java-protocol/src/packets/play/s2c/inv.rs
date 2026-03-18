use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_CONTAINER_CONTENT: i32 = 0x13;
pub const ID_CONTAINER_SLOT:    i32 = 0x15;
pub const ID_OPEN_SCREEN:       i32 = 0x31;
pub const ID_CLOSE_SCREEN:      i32 = 0x12;
pub const ID_SET_CARRY_ITEM:    i32 = 0x53;

#[derive(Debug, Clone)]
pub struct SlotData {
    pub a: bool,
    pub b: i32,
    pub c: i8,
    pub d: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct ContainerContent {
    pub a: u8,
    pub b: i32,
    pub c: Vec<SlotData>,
    pub d: SlotData,
}

#[derive(Debug, Clone)]
pub struct ContainerSlot {
    pub a: u8,
    pub b: i32,
    pub c: i16,
    pub d: SlotData,
}

#[derive(Debug, Clone)]
pub struct OpenScreen { pub a: i32, pub b: i32, pub c: String }

#[derive(Debug, Clone)]
pub struct SetCarryItem { pub a: i16 }

fn write_slot(cx: &mut Enc, s: &SlotData) {
    cx.bool(s.a);
    if s.a {
        cx.var32(s.b); cx.i8(s.c);
        if let Some(ref n) = s.d { cx.bytes(n); }
    }
}

fn read_slot(cx: &mut Dec) -> R<SlotData> {
    let a = cx.bool()?;
    if !a { return Ok(SlotData { a, b: 0, c: 0, d: None }); }
    let b = cx.var32()?; let c = cx.i8()?;
    Ok(SlotData { a, b, c, d: None })
}

impl ContainerContent {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CONTAINER_CONTENT);
        cx.u8(self.a); cx.var32(self.b);
        cx.var32(self.c.len() as i32);
        for s in &self.c { write_slot(&mut cx, s); }
        write_slot(&mut cx, &self.d);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.u8()?; let bb = cx.var32()?;
        let n = cx.var32()? as usize;
        let mut c = Vec::with_capacity(n);
        for _ in 0..n { c.push(read_slot(&mut cx)?); }
        let d = read_slot(&mut cx)?;
        Ok(Self { a, b: bb, c, d })
    }
}

impl ContainerSlot {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CONTAINER_SLOT);
        cx.u8(self.a); cx.var32(self.b); cx.i16(self.c);
        write_slot(&mut cx, &self.d); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.u8()?; let bb = cx.var32()?; let c = cx.i16()?;
        Ok(Self { a, b: bb, c, d: read_slot(&mut cx)? })
    }
}

impl OpenScreen {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_OPEN_SCREEN);
        cx.var32(self.a); cx.var32(self.b); cx.str(&self.c); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.var32()?, b: cx.var32()?, c: cx.str()? })
    }
}

impl SetCarryItem {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_SET_CARRY_ITEM); cx.i16(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.i16()? })
    }
}
