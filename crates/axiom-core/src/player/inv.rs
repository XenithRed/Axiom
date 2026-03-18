use serde::{Deserialize, Serialize};
use crate::error::{Err, R};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemStack {
    pub a: i32,
    pub b: u8,
    pub c: Option<Vec<u8>>,
}

impl ItemStack {
    pub fn new(id: i32, count: u8) -> Self {
        Self { a: id, b: count, c: None }
    }

    pub fn empty() -> Self { Self::default() }
    pub fn is_empty(&self) -> bool { self.a == 0 || self.b == 0 }
    pub fn id(&self) -> i32 { self.a }
    pub fn count(&self) -> u8 { self.b }
    pub fn nbt(&self) -> Option<&[u8]> { self.c.as_deref() }
}

pub const INV_SIZE: usize = 46;
pub const HOTBAR_START: usize = 36;
pub const HOTBAR_END: usize = 44;
pub const OFFHAND: usize = 45;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    a: [ItemStack; INV_SIZE],
    b: u8,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            a: std::array::from_fn(|_| ItemStack::empty()),
            b: 0,
        }
    }

    pub fn get(&self, slot: usize) -> R<&ItemStack> {
        self.a.get(slot).ok_or(Err::BadSlot(slot))
    }

    pub fn get_mut(&mut self, slot: usize) -> R<&mut ItemStack> {
        self.a.get_mut(slot).ok_or(Err::BadSlot(slot))
    }

    pub fn set(&mut self, slot: usize, item: ItemStack) -> R<()> {
        *self.a.get_mut(slot).ok_or(Err::BadSlot(slot))? = item;
        Ok(())
    }

    pub fn held_slot(&self) -> usize { HOTBAR_START + self.b as usize }

    pub fn held(&self) -> &ItemStack {
        &self.a[self.held_slot()]
    }

    pub fn set_held_slot(&mut self, n: u8) {
        self.b = n.min(8);
    }

    pub fn offhand(&self) -> &ItemStack { &self.a[OFFHAND] }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &ItemStack)> {
        self.a.iter().enumerate()
    }

    pub fn first_empty(&self) -> Option<usize> {
        self.a.iter().position(|s| s.is_empty())
    }
}

impl Default for Inventory {
    fn default() -> Self { Self::new() }
}
