use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_INTERACT:    u8 = 0x21;
pub const ID_BLOCK_BREAK: u8 = 0x17;
pub const ID_BLOCK_PLACE: u8 = 0x1C;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InteractAction { StopRiding = 3, Interact = 4, Attack = 5, ItemInteract = 6 }

impl InteractAction {
    pub fn from_u8(n: u8) -> Self {
        match n { 3 => Self::StopRiding, 5 => Self::Attack, 6 => Self::ItemInteract, _ => Self::Interact }
    }
}

#[derive(Debug, Clone)]
pub struct Interact {
    pub a: InteractAction,
    pub b: u64,
    pub c: f32, pub d: f32, pub e: f32,
}

#[derive(Debug, Clone)]
pub struct BlockBreak {
    pub a: i32, pub b: i32, pub c: i32,
    pub d: u32,
    pub e: i32,
}

#[derive(Debug, Clone)]
pub struct BlockPlace {
    pub a: i32, pub b: i32, pub c: i32,
    pub d: u32,
    pub e: i32,
    pub f: f32, pub g: f32, pub h: f32,
    pub i: bool,
}

impl Interact {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_INTERACT);
        cx.u8(self.a as u8);
        cx.varint64(self.b);
        cx.f32(self.c); cx.f32(self.d); cx.f32(self.e);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: InteractAction::from_u8(cx.u8()?),
            b: cx.varint64()?,
            c: cx.f32()?, d: cx.f32()?, e: cx.f32()?,
        })
    }
}

impl BlockBreak {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_BLOCK_BREAK);
        cx.varinti(self.a); cx.varinti(self.b); cx.varinti(self.c);
        cx.varint(self.d);
        cx.varinti(self.e);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.varinti()?, b: cx.varinti()?, c: cx.varinti()?,
            d: cx.varint()?,
            e: cx.varinti()?,
        })
    }
}

impl BlockPlace {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID_BLOCK_PLACE);
        cx.varinti(self.a); cx.varinti(self.b); cx.varinti(self.c);
        cx.varint(self.d);
        cx.varinti(self.e);
        cx.f32(self.f); cx.f32(self.g); cx.f32(self.h);
        cx.bool(self.i);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a: cx.varinti()?, b: cx.varinti()?, c: cx.varinti()?,
            d: cx.varint()?,
            e: cx.varinti()?,
            f: cx.f32()?, g: cx.f32()?, h: cx.f32()?,
            i: cx.bool()?,
        })
    }
}
