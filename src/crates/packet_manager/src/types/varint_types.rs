use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VarInt {
    pub value: i32,
    pub len: usize,
}

impl VarInt {
    pub fn new(value: i32) -> Self {
        let len = match value {
            -128..=127 => 1,
            -16_384..=16_383 => 2,
            -2_097_152..=2_097_151 => 3,
            -268_435_456..=268_435_455 => 4,
            _ => 5,
        };

        Self { value, len }
    }

    pub fn to_i32(&self) -> i32 {
        self.value
    }

    pub fn length(&self) -> usize {
        self.len
    }

}

impl Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        Self { value, len: 0 }
    }
}

impl Into<usize> for VarInt {
    fn into(self) -> usize {
        self.value as usize
    }
}

impl From<VarInt> for i32 {
    fn from(value: VarInt) -> Self {
        value.value
    }
}

impl From<VarInt> for u32 {
    fn from(value: VarInt) -> Self {
        value.value as u32
    }
}
