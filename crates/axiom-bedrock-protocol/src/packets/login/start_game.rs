use bytes::Bytes;
use crate::codec::{Dec, Enc};
use crate::error::R;

pub const ID: u8 = 0x0B;

#[derive(Debug, Clone)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }

#[derive(Debug, Clone)]
pub struct Vec2 { pub x: f32, pub y: f32 }

#[derive(Debug, Clone)]
pub struct StartGame {
    pub a:  i64,
    pub b:  i64,
    pub c:  i32,
    pub d:  Vec3,
    pub e:  Vec2,
    pub f:  i32,
    pub g:  i32,
    pub h:  i64,
    pub i:  bool,
    pub j:  bool,
    pub k:  bool,
    pub l:  u32,
    pub m:  i32,
    pub n:  i32,
    pub o:  String,
    pub p:  String,
    pub q:  bool,
    pub r:  i64,
}

impl StartGame {
    pub fn encode(&self) -> Bytes {
        let mut cx = Enc::new();
        cx.u8(ID);
        cx.varinti64(self.a);
        cx.varinti64(self.b);
        cx.varinti(self.c);
        cx.f32(self.d.x); cx.f32(self.d.y); cx.f32(self.d.z);
        cx.f32(self.e.x); cx.f32(self.e.y);
        cx.varinti(self.f);
        cx.varinti(self.g);
        cx.varinti64(self.h);
        cx.bool(self.i);
        cx.bool(self.j);
        cx.bool(self.k);
        cx.varint(self.l);
        cx.varinti(self.m);
        cx.varinti(self.n);
        cx.str(&self.o);
        cx.str(&self.p);
        cx.bool(self.q);
        cx.varinti64(self.r);
        cx.finish()
    }

    pub fn decode(b: Bytes) -> R<Self> {
        let mut cx = Dec::new(b);
        cx.u8()?;
        Ok(Self {
            a:  cx.varinti64()?,
            b:  cx.varinti64()?,
            c:  cx.varinti()?,
            d:  Vec3 { x: cx.f32()?, y: cx.f32()?, z: cx.f32()? },
            e:  Vec2 { x: cx.f32()?, y: cx.f32()? },
            f:  cx.varinti()?,
            g:  cx.varinti()?,
            h:  cx.varinti64()?,
            i:  cx.bool()?,
            j:  cx.bool()?,
            k:  cx.bool()?,
            l:  cx.varint()?,
            m:  cx.varinti()?,
            n:  cx.varinti()?,
            o:  cx.str()?,
            p:  cx.str()?,
            q:  cx.bool()?,
            r:  cx.varinti64()?,
        })
    }
}
