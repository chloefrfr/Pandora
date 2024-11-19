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

    pub fn to_be_bytes(&self) -> [u8; 5] {
        let mut buf = [0u8; 5];
        buf[0] = (self.value & 0x000000FF) as u8;
        buf[1] = ((self.value & 0x0000FF00) >> 8) as u8;
        buf[2] = ((self.value & 0x00FF0000) >> 16) as u8;
        buf[3] = ((self.value & 0xFF000000u32 as i32) >> 24) as u8;
        buf[4] = ((self.value & 0xFF000000u32 as i32) >> 24) as u8;
        buf
    }
}

impl Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<VarInt> for i32 {
    fn from(varint: VarInt) -> i32 {
        varint.value
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> VarInt {
        VarInt::new(value)
    }
}

impl Into<usize> for VarInt {
    fn into(self) -> usize {
        self.value as usize
    }
}
