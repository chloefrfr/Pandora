use bytes::{Buf, BufMut, BytesMut};
use log::error;
use nbt::Blob;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

pub struct PacketManager {
    buffer: BytesMut,
    offset: usize,
}

impl PacketManager {
    #[inline]
    pub fn new(buffer: BytesMut, offset: usize) -> Self {
        Self { buffer, offset }
    }

    #[inline(always)]
    fn ensure_available_bytes(&self, required: usize) -> bool {
        self.buffer.remaining() >= required
    }

    #[inline(always)]
    pub fn read_boolean(&mut self) -> bool {
        if self.ensure_available_bytes(1) {
            self.read_byte() != 0
        } else {
            false
        }
    }

    pub fn read_var_int_checked(&mut self) -> Option<i32> {
        let mut value = 0;
        let mut shift = 0;

        for _ in 0..5 {
            if self.ensure_available_bytes(1) {
                let byte = self.read_unsigned_byte();
                value |= ((byte & 0x7F) as i32) << shift;
                if (byte & 0x80) == 0 {
                    return Some(value);
                }
                shift += 7;
            } else {
                return None;
            }
        }
        None
    }

    #[inline(always)]
    pub fn read_byte(&mut self) -> i8 {
        if self.ensure_available_bytes(1) {
            self.buffer.get_i8()
        } else {
            0
        }
    }

    pub fn read_float(&mut self) -> f32 {
        if self.ensure_available_bytes(4) {
            self.buffer.get_f32()
        } else {
            0.0
        }
    }

    #[inline(always)]
    pub fn read_unsigned_byte(&mut self) -> u8 {
        if self.ensure_available_bytes(1) {
            self.buffer.get_u8()
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn read_short(&mut self) -> i16 {
        if self.ensure_available_bytes(2) {
            self.buffer.get_i16()
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn read_unsigned_short(&mut self) -> u16 {
        if self.ensure_available_bytes(2) {
            self.buffer.get_u16()
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn read_int(&mut self) -> i32 {
        if self.ensure_available_bytes(4) {
            self.buffer.get_i32()
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn read_long(&mut self) -> i64 {
        if self.ensure_available_bytes(8) {
            self.buffer.get_i64()
        } else {
            0
        }
    }

    pub fn read_var_int(&mut self) -> i32 {
        let mut value = 0;
        let mut shift = 0;

        for _ in 0..5 {
            if self.ensure_available_bytes(1) {
                let byte = self.read_unsigned_byte();
                value |= ((byte & 0x7F) as i32) << shift;
                if (byte & 0x80) == 0 {
                    return value;
                }
                shift += 7;
            } else {
                return 0;
            }
        }
        0
    }

    pub fn append_blob(&mut self, blob: &Blob) {
        let serialized_blob = serde_json::to_vec(blob).unwrap();
        let length = serialized_blob.len();

        self.buffer.extend_from_slice(&serialized_blob);
        self.add_offset(length, true);
    }

    pub fn read_double(&mut self) -> f64 {
        if self.ensure_available_bytes(8) {
            self.buffer.get_f64()
        } else {
            0.0
        }
    }

    pub fn read_string(&mut self) -> String {
        let length = self.read_var_int() as usize;
        if self.ensure_available_bytes(length) {
            let slice = self.buffer.split_to(length);
            String::from_utf8(slice.to_vec()).unwrap_or_default()
        } else {
            String::new()
        }
    }

    pub fn read_uuid(&mut self) -> String {
        let high = self.read_long();
        let low = self.read_long();
        format!("{:016x}{:016x}", high, low)
    }

    #[inline(always)]
    pub fn write_boolean(&mut self, value: bool) {
        self.write_byte(if value { 1 } else { 0 });
    }

    #[inline(always)]
    pub fn write_byte(&mut self, value: i8) {
        self.buffer.put_i8(value);
    }

    #[inline(always)]
    pub fn write_unsigned_byte(&mut self, value: u8) {
        self.buffer.put_u8(value);
    }

    #[inline(always)]
    pub fn write_short(&mut self, value: i16) {
        self.buffer.put_i16(value);
    }

    #[inline(always)]
    pub fn write_unsigned_short(&mut self, value: u16) {
        self.buffer.put_u16(value);
    }

    #[inline(always)]
    pub fn write_double(&mut self, value: f64) {
        self.buffer.put_f64(value);
    }

    #[inline(always)]
    pub fn write_float(&mut self, value: f32) {
        self.buffer.put_f32(value);
    }

    #[inline(always)]
    pub fn write_int(&mut self, value: i32) {
        self.buffer.put_i32(value);
    }

    #[inline(always)]
    pub fn write_long(&mut self, value: BigInt) {
        if let Some(val) = value.to_i64() {
            self.buffer.put_i64(val);
        } else {
            error!("Failed to convert BigInt to i64");
        }
    }

    pub fn write_string(&mut self, value: &str) {
        self.write_var_int(value.len() as i32);
        self.buffer.extend_from_slice(value.as_bytes());
    }

    pub fn write_var_int(&mut self, mut value: i32) {
        while value & !0x7F != 0 {
            self.write_unsigned_byte((value & 0x7F) as u8 | 0x80);
            value >>= 7;
        }
        self.write_unsigned_byte(value as u8);
    }

    pub fn write_var_long(&mut self, mut value: i64) {
        while value & !0x7F != 0 {
            self.write_unsigned_byte((value & 0x7F) as u8 | 0x80);
            value >>= 7;
        }
        self.write_unsigned_byte(value as u8);
    }

    pub fn write_uuid(&mut self, value: &str) {
        let uuid = value.replace("-", "");
        let high = u128::from_str_radix(&uuid[..16], 16).expect("Invalid UUID format");
        let low = u128::from_str_radix(&uuid[16..], 16).expect("Invalid UUID format");
        self.write_long((high as i64).into());
        self.write_long((low as i64).into());
    }

    pub fn add_offset(&mut self, size: usize, retval: bool) -> usize {
        if retval {
            self.offset += size;
            self.offset
        } else {
            let current_offset = self.offset;
            self.offset += size;
            current_offset
        }
    }

    #[inline(always)]
    pub fn build_packet(&mut self, id: i32) -> BytesMut {
        let mut pk_id = PacketManager::new(BytesMut::new(), 0);
        pk_id.write_var_int(id);

        let mut pk_length = PacketManager::new(BytesMut::new(), 0);
        pk_length.write_var_int(pk_id.buffer.len() as i32 + self.buffer.len() as i32);

        let mut pk = BytesMut::with_capacity(
            pk_length.buffer.len() + pk_id.buffer.len() + self.buffer.len(),
        );
        pk.extend_from_slice(&pk_length.buffer);
        pk.extend_from_slice(&pk_id.buffer);
        pk.extend_from_slice(&self.buffer);
        pk
    }

    pub fn append(&mut self, data: &BytesMut) {
        self.buffer.extend(data);
        self.add_offset(data.len(), true);
    }

    pub fn extend_from_slice(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        self.add_offset(data.len(), true);
    }

    pub fn get_buffer(&self) -> &BytesMut {
        &self.buffer
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }
}
