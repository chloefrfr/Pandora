use bytes::{Buf, BufMut, BytesMut};
use log::error;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use tokio::io::AsyncReadExt;

use crate::types::varint_types::VarInt;

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

    pub async fn read_boolean(&mut self) -> bool {
        self.ensure_available_bytes(1) && self.read_byte().await != 0
    }

    pub async fn read_byte(&mut self) -> i8 {
        self.buffer.get_i8()
    }

    pub async fn read_unsigned_byte(&mut self) -> u8 {
        self.buffer.get_u8()
    }

    pub async fn read_short(&mut self) -> i16 {
        self.buffer.get_i16()
    }

    pub async fn read_unsigned_short(&mut self) -> u16 {
        self.buffer.get_u16()
    }

    pub async fn read_int(&mut self) -> i32 {
        self.buffer.get_i32()
    }

    pub async fn read_long(&mut self) -> i64 {
        self.buffer.get_i64()
    }

    pub async fn read_float(&mut self) -> f32 {
        self.buffer.get_f32()
    }

    pub async fn read_double(&mut self) -> f64 {
        self.buffer.get_f64()
    }

    pub async fn read_var_int<T>(cursor: &mut T) -> Result<VarInt, String>
    where
        T: tokio::io::AsyncRead + tokio::io::AsyncSeek + Unpin,
    {
        let mut result: i32 = 0;

        let mut cursor = Box::pin(cursor);

        for i in 0..5 {
            let mut byte = [0u8; 1];
            cursor
                .read_exact(&mut byte)
                .await
                .map_err(|e| e.to_string())?;
            let byte = byte[0];

            result |= ((byte as i32) & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(VarInt {
                    value: result,
                    len: i + 1,
                });
            }
        }

        Err("VarInt too long".to_string())
    }

    pub async fn read_var_int_checked(&mut self) -> Option<i32> {
        let mut value = 0;
        let mut shift = 0;

        for _ in 0..5 {
            if self.ensure_available_bytes(1) {
                let byte = self.read_unsigned_byte().await;
                value |= ((byte & 0x7F) as i32) << shift;
                if byte & 0x80 == 0 {
                    return Some(value);
                }
                shift += 7;
            } else {
                return None;
            }
        }

        None
    }

    pub async fn read_string(&mut self) -> Result<String, String> {
        let length = self
            .read_var_int_checked()
            .await
            .ok_or_else(|| "Failed to read string length (VarInt)".to_string())?;

        if self.ensure_available_bytes(length as usize) {
            let slice = self.buffer.split_to(length as usize);
            String::from_utf8(slice.to_vec()).map_err(|e| e.to_string())
        } else {
            Err("Insufficient bytes available to read string".to_string())
        }
    }

    pub async fn read_uuid(&mut self) -> Result<String, String> {
        let high = self.read_long().await;
        let low = self.read_long().await;
        Ok(format!("{:016x}{:016x}", high, low))
    }

    pub fn write_boolean(&mut self, value: bool) {
        self.write_byte(if value { 1 } else { 0 });
    }

    pub fn write_byte(&mut self, value: i8) {
        self.buffer.put_i8(value);
    }

    pub fn write_var_int(&mut self, value: &VarInt) {
        let value_i32 = value.to_i32();

        let mut value = value_i32;
        while value & !0x7F != 0 {
            self.write_unsigned_byte((value & 0x7F) as u8 | 0x80);
            value >>= 7;
        }
        self.write_unsigned_byte(value as u8);
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

    pub fn write_long(&mut self, value: BigInt) {
        if let Some(val) = value.to_i64() {
            self.buffer.put_i64(val);
        } else {
            error!("Failed to convert BigInt to i64");
        }
    }

    pub fn write_float(&mut self, value: f32) {
        self.buffer.put_f32(value);
    }

    pub fn write_double(&mut self, value: f64) {
        self.buffer.put_f64(value);
    }

    pub fn write_string(&mut self, value: &str) {
        self.write_var_int_checked(value.len() as i32);
        self.buffer.extend_from_slice(value.as_bytes());
    }

    pub fn write_var_int_checked(&mut self, mut value: i32) {
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

    pub fn write_uuid(&mut self, value: &str) -> Result<(), String> {
        let uuid = value.replace("-", "");
        if uuid.len() != 32 {
            return Err("Invalid UUID format".to_string());
        }

        let high = u128::from_str_radix(&uuid[..16], 16).map_err(|_| "Invalid UUID format")?;
        let low = u128::from_str_radix(&uuid[16..], 16).map_err(|_| "Invalid UUID format")?;
        self.write_long((high as i64).into());
        self.write_long((low as i64).into());
        Ok(())
    }

    pub fn add_offset(&mut self, size: usize, update_offset: bool) -> usize {
        if update_offset {
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
        pk_id.write_var_int_checked(id);

        let mut pk_length: PacketManager = PacketManager::new(BytesMut::new(), 0);
        pk_length.write_var_int_checked(pk_id.buffer.len() as i32 + self.buffer.len() as i32);

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
