use bytes::{Buf, BufMut, BytesMut};
use std::convert::TryInto;

pub struct PacketManager {
    buffer: BytesMut,
    offset: usize,
}

impl PacketManager {
    pub fn new(buffer: BytesMut, offset: usize) -> Self {
        Self { buffer, offset }
    }

    pub fn read_boolean(&mut self) -> bool {
        self.read_byte() != 0
    }

    pub fn read_byte(&mut self) -> i8 {
        self.buffer.get_i8()
    }

    pub fn read_unsigned_byte(&mut self) -> u8 {
        self.buffer.get_u8()
    }

    pub fn read_short(&mut self) -> i16 {
        self.buffer.get_i16()
    }

    pub fn read_unsigned_short(&mut self) -> u16 {
        self.buffer.get_u16()
    }

    pub fn read_int(&mut self) -> i32 {
        self.buffer.get_i32()
    }

    pub fn read_long(&mut self) -> i64 {
        self.buffer.get_i64()
    }

    pub fn read_unsigned_long(&mut self) -> u64 {
        self.buffer.get_u64()
    }

    pub fn read_float(&mut self) -> f32 {
        self.buffer.get_f32()
    }

    pub fn read_double(&mut self) -> f64 {
        self.buffer.get_f64()
    }

    pub fn read_string(&mut self) -> String {
        let length = self.read_var_int();
        let slice = self.buffer.split_to(length as usize);
        String::from_utf8(slice.to_vec()).expect("Invalid UTF-8 sequence")
    }

    pub fn read_var_int(&mut self) -> i32 {
        let mut value = 0;
        let mut shift = 0;

        loop {
            let byte = self.read_unsigned_byte();
            value |= ((byte & 0x7F) as i32) << shift;
            if shift >= 28 {
                panic!("VarInt too big");
            }
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        value
    }

    pub fn read_var_long(&mut self) -> i64 {
        let mut value = 0;
        let mut shift = 0;

        loop {
            let byte = self.read_unsigned_byte();
            value |= ((byte & 0x7F) as i64) << shift;
            if shift >= 70 {
                panic!("VarLong too big");
            }
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        value
    }

    pub fn read_uuid(&mut self) -> String {
        let high = self.read_long();
        let low = self.read_long();
        format!("{:x}{:x}", high, low)
    }

    pub fn write_boolean(&mut self, value: bool) {
        self.write_byte(if value { 1 } else { 0 });
    }

    pub fn write_byte(&mut self, value: i8) {
        self.buffer.put_i8(value);
    }

    pub fn write_unsigned_byte(&mut self, value: u8) {
        self.buffer.put_u8(value);
    }

    pub fn write_short(&mut self, value: i16) {
        self.buffer.put_i16(value);
    }

    pub fn write_unsigned_short(&mut self, value: u16) {
        self.buffer.put_u16(value);
    }

    pub fn write_int(&mut self, value: i32) {
        self.buffer.put_i32(value);
    }

    pub fn write_long(&mut self, value: i64) {
        self.buffer.put_i64(value);
    }

    pub fn write_unsigned_long(&mut self, value: u64) {
        self.buffer.put_u64(value);
    }

    pub fn write_float(&mut self, value: f32) {
        self.buffer.put_f32(value);
    }

    pub fn write_double(&mut self, value: f64) {
        self.buffer.put_f64(value);
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
        let high = i64::from_str_radix(&uuid[0..16], 16).expect("Invalid UUID format");
        let low = i64::from_str_radix(&uuid[16..32], 16).expect("Invalid UUID format");
        self.write_long(high);
        self.write_long(low);
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

    pub fn build_packet(&mut self, id: i32) -> BytesMut {
        let mut pk_id = PacketManager::new(BytesMut::new(), 0);
        pk_id.write_var_int(id);
        let mut pk_length = PacketManager::new(BytesMut::new(), 0);
        pk_length.write_var_int(pk_id.buffer.len() as i32 + self.buffer.len() as i32);
        let mut pk = BytesMut::new();
        pk.extend_from_slice(&pk_length.buffer);
        pk.extend_from_slice(&pk_id.buffer);
        pk.extend_from_slice(&self.buffer);
        pk
    }

    pub fn append(&mut self, data: &BytesMut) {
        self.buffer.extend(data);
        self.add_offset(data.len(), true);
    }

    pub fn get_buffer(&self) -> &BytesMut {
        &self.buffer
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }
}
