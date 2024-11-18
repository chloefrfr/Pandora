use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VarInt(pub i32);

impl VarInt {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn to_i32(&self) -> i32 {
        self.0
    }
}

impl Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Into<usize> for VarInt {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl From<VarInt> for i32 {
    fn from(value: VarInt) -> Self {
        value.0
    }
}

impl From<VarInt> for u32 {
    fn from(value: VarInt) -> Self {
        value.0 as u32
    }
}
