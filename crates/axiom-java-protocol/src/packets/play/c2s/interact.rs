use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID_INTERACT:      i32 = 0x13;
pub const ID_USE_ITEM_ON:   i32 = 0x36;
pub const ID_USE_ITEM:      i32 = 0x37;
pub const ID_PLAYER_ACTION: i32 = 0x24;
pub const ID_ATTACK_BLOCK:  i32 = 0x01;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum InteractType { Interact = 0, Attack = 1, InteractAt = 2 }

impl InteractType {
    pub fn from_i32(n: i32) -> Self {
        match n { 1 => Self::Attack, 2 => Self::InteractAt, _ => Self::Interact }
    }
}

#[derive(Debug, Clone)]
pub struct Interact {
    pub a: i32,
    pub b: InteractType,
    pub c: Option<(f32, f32, f32)>,
    pub d: Option<i32>,
    pub e: bool,
}

#[derive(Debug, Clone)]
pub struct UseItemOn {
    pub a: i32, pub b: i16, pub c: i32,
    pub d: i32,
    pub e: f32, pub f: f32, pub g: f32,
    pub h: bool,
    pub i: i32,
}

#[derive(Debug, Clone)]
pub struct UseItem { pub a: i32, pub b: i32, pub c: i64 }

#[derive(Debug, Clone)]
pub struct PlayerAction {
    pub a: i32,
    pub b: i32, pub c: i16, pub d: i32,
    pub e: i32,
    pub f: i32,
}

impl Interact {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_INTERACT);
        cx.var32(self.a);
        cx.var32(self.b as i32);
        if self.b == InteractType::InteractAt {
            if let Some((x, y, z)) = self.c { cx.f32(x); cx.f32(y); cx.f32(z); }
        }
        if let Some(hand) = self.d { cx.var32(hand); }
        cx.bool(self.e);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a  = cx.var32()?;
        let bt = InteractType::from_i32(cx.var32()?);
        let c  = if bt == InteractType::InteractAt {
            Some((cx.f32()?, cx.f32()?, cx.f32()?))
        } else { None };
        let d = if bt != InteractType::Attack { Some(cx.var32()?) } else { None };
        let e = cx.bool()?;
        Ok(Self { a, b: bt, c, d, e })
    }
}

impl UseItemOn {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_USE_ITEM_ON);
        cx.var32(self.a);
        cx.pos(self.b as i32, self.c as i16, self.d);
        cx.var32(self.d);
        cx.f32(self.e); cx.f32(self.f); cx.f32(self.g);
        cx.bool(self.h);
        cx.var32(self.i);
        cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        let a = cx.var32()?;
        let (bx, by, bz) = cx.pos()?;
        let d = cx.var32()?;
        Ok(Self {
            a, b: bx as i32, c: by, d: bz,
            e: cx.f32()?, f: cx.f32()?, g: cx.f32()?,
            h: cx.bool()?, i: cx.var32()?,
        })
    }
}

impl UseItem {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new(); cx.var32(ID_USE_ITEM);
        cx.var32(self.a); cx.var32(self.b); cx.var64(self.c); cx.finish()
    }
    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b); cx.var32()?;
        Ok(Self { a: cx.var32()?, b: cx.var32()?, c: cx.var64()? })
    }
}
