use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_INV_CONTENT:  u8 = 0x31;
pub const ID_INV_SLOT:     u8 = 0x32;
pub const ID_CONTAINER_OPEN: u8 = 0x2E;
pub const ID_CRAFTING_DATA: u8 = 0x34;
pub const ID_CREATIVE_CONTENT: u8 = 0x92;

#[derive(Debug, Clone)]
pub struct ItemStack {
    pub a: i16,
    pub b: i16,
    pub c: i32,
    pub d: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct InvContent {
    pub a: u32,
    pub b: Vec<ItemStack>,
}

#[derive(Debug, Clone)]
pub struct InvSlot {
    pub a: u32,
    pub b: u32,
    pub c: ItemStack,
}

#[derive(Debug, Clone)]
pub struct ContainerOpen {
    pub a: i8,
    pub b: u8,
    pub c: i32, pub d: i32, pub e: i32,
    pub f: i64,
}

fn write_item(cx: &mut Enc, it: &ItemStack) {
    cx.i16(it.a);
    if it.a == 0 { return; }
    cx.i16(it.b);
    cx.i32(it.c);
    if let Some(ref nbt) = it.d {
        cx.i16(nbt.len() as i16);
        cx.bytes(nbt);
    } else {
        cx.i16(0);
    }
    cx.i32(0);
    cx.i32(0);
}

fn read_item(cx: &mut Dec) -> R<ItemStack> {
    let a = cx.i16()?;
    if a == 0 { return Ok(ItemStack { a, b: 0, c: 0, d: None }); }
    let b = cx.i16()?;
    let c = cx.i32()?;
    let nbt_len = cx.i16()?;
    let d = if nbt_len > 0 {
        Some(cx.slice(nbt_len as usize)?)
    } else { None };
    let _ = cx.i32()?;
    let _ = cx.i32()?;
    Ok(ItemStack { a, b, c, d })
}

impl InvContent {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_INV_CONTENT);
        cx.varint(self.a);
        cx.varint(self.b.len() as u32);
        for it in &self.b { write_item(&mut cx, it); }
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let a = cx.varint()?;
        let n = cx.varint()? as usize;
        let mut bv = Vec::with_capacity(n);
        for _ in 0..n { bv.push(read_item(&mut cx)?); }
        Ok(Self { a, b: bv })
    }
}

impl InvSlot {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_INV_SLOT);
        cx.varint(self.a);
        cx.varint(self.b);
        write_item(&mut cx, &self.c);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        let a = cx.varint()?;
        let bb = cx.varint()?;
        let c = read_item(&mut cx)?;
        Ok(Self { a, b: bb, c })
    }
}

impl ContainerOpen {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_CONTAINER_OPEN);
        cx.i8(self.a);
        cx.u8(self.b);
        cx.varinti(self.c); cx.varinti(self.d); cx.varinti(self.e);
        cx.varinti64(self.f);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.i8()?,
            b: cx.u8()?,
            c: cx.varinti()?, d: cx.varinti()?, e: cx.varinti()?,
            f: cx.varinti64()?,
        })
    }
}
