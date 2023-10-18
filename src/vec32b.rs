use std::arch::x86_64::__mmask32;
use std::ops::Index;

#[derive(Clone, Copy, Debug)]
pub struct Vec32b {
    mm: __mmask32,
}

impl Vec32b {
    pub fn new() -> Self {
        Self {
            mm: 0,
        }
    }

    pub fn insert(mut self, index: usize, value: bool) -> Self {
        self.mm = self.mm & !(1u32 << index) | (value as u32) << index;
        self
    }

    pub fn extract(&self, index: usize) -> bool {
        ((self.mm >> index) & 1) != 0
    }
}

impl Index<usize> for Vec32b {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if self.extract(index) {
            &true
        } else {
            &false
        }
    }
}

impl From<Vec32b> for __mmask32 {
    fn from(val: Vec32b) -> Self {
        val.mm
    }
}

impl From<bool> for Vec32b {
    fn from(value: bool) -> Self {
        (-(value as i32) as __mmask32).into()
    }
}

impl From<__mmask32> for Vec32b {
    fn from(value: __mmask32) -> Self {
        Self {
            mm: value,
        }
    }
}

impl Default for Vec32b {
    fn default() -> Self {
        Self::new()
    }
}
