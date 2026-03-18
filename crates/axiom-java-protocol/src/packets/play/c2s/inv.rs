use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_CLICK_CONTAINER:  i32 = 0x0C;
pub const ID_CLOSE_CONTAINER:  i32 = 0x0D;
pub const ID_SLOT_CHANGED:     i32 = 0x31;
pub const ID_PICK_ITEM:        i32 = 0x22;
pub const ID_DROP_ITEM:        i32 = 0x1A; 
pub const ID_SWAP_HANDS:       i32 = 0x35;
pub const ID_CREATIVE_INV:     i32 = 0x3B;

#[derive(Debug, Clone)]
pub struct SlotData {
    pub a: i16,
    pub b: i8,
    pub c: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct ClickContainer {
    pub a: u8,
    pub b: i32,
    pub c: i16,
    pub d: i8,
    pub e: i32,
    pub f: Vec<(i16, SlotData)>,
    pub g: SlotData,
}

#[derive(Debug, Clone)]
pub struct CloseContainer { pub a: u8 }

#[derive(Debug, Clone)]
pub struct CreativeInv { pub a: i16, pub b: SlotData }

#[derive(Debug, Clone)]
pub struct DropItem { pub a: bool }

fn write_slot(cx: &mut Enc, s: &SlotData) {
    cx.bool(s.c.is_some());
    if let Some(ref nbt) = s.c {
        cx.i16(s.a); cx.i8(s.b);
        cx.bytes(nbt);
    }
}

fn read_slot(cx: &mut Dec) -> R<SlotData> {
    let present = cx.bool()?;
    if !present { return Ok(SlotData { a: 0, b: 0, c: None }); }
    let a = cx.i16()?; let b = cx.i8()?;
    let c = Some(cx.rest());
    Ok(SlotData { a, b, c })
}

impl ClickContainer {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CLICK_CONTAINER);
        cx.u8(self.a); cx.var32(self.b);
        cx.i16(self.c); cx.i8(self.d); cx.var32(self.e);
        cx.var32(self.f.len() as i32);
        for (slot, data) in &self.f { cx.i16(*slot); write_slot(&mut cx, data); }
        write_slot(&mut cx, &self.g);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.u8()?; let bb = cx.var32()?;
        let c = cx.i16()?; let d = cx.i8()?; let e = cx.var32()?;
        let n = cx.var32()? as usize;
        let mut f = Vec::with_capacity(n);
        for _ in 0..n { f.push((cx.i16()?, read_slot(&mut cx)?)); }
        let g = read_slot(&mut cx)?;
        Ok(Self { a, b: bb, c, d, e, f, g })
    }
}

impl CloseContainer {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CLOSE_CONTAINER); cx.u8(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.u8()? })
    }
}

impl CreativeInv {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_CREATIVE_INV);
        cx.i16(self.a); write_slot(&mut cx, &self.b); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.i16()?, b: read_slot(&mut cx)? })
    }
}

impl DropItem {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_DROP_ITEM); cx.bool(self.a); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?; Ok(Self { a: cx.bool()? })
    }
}
