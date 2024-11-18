use std::{error::Error, io::Read};

use bytes::BytesMut;
use packet_manager::{types::varint_types::VarInt, PacketManager};

pub trait Decode: Sized {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read;
}

impl Decode for bool {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let mut buf = [0u8; 1];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read bool")?;
        Ok(buf[0] != 0)
    }
}

impl Decode for u8 {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let mut buf = [0u8; 1];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read u8")?;
        Ok(u8::from_le_bytes(buf))
    }
}

impl Decode for u16 {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let mut buf = [0u8; 2];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read u16")?;
        Ok(u16::from_le_bytes(buf))
    }
}

impl Decode for u32 {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let mut buf = [0u8; 4];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read u32")?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl Decode for i16 {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let mut buf = [0u8; 2];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read i16")?;
        Ok(i16::from_le_bytes(buf))
    }
}

impl Decode for i32 {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let mut buf = [0u8; 4];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read i32")?;
        Ok(i32::from_le_bytes(buf))
    }
}

impl Decode for f32 {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let mut buf = [0u8; 4];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read f32")?;
        Ok(f32::from_le_bytes(buf))
    }
}

impl Decode for f64 {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let mut buf = [0u8; 8];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read f64")?;
        Ok(f64::from_le_bytes(buf))
    }
}

impl Decode for String {
    fn decode<T>(bytes: &mut T) -> Result<std::string::String, Box<dyn std::error::Error>>
    where
        T: Read,
    {
        let mut packet_manager = PacketManager::new(BytesMut::new(), 0);
        let remaining_bytes: VarInt = packet_manager.read_var_int(bytes)?;

        let mut string = vec![0u8; remaining_bytes.to_i32() as usize];
        bytes.read_exact(&mut string)?;
        String::from_utf8(string).map_err(|e| e.into())
    }
}

impl Decode for Vec<u8> {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let len = u16::decode(bytes)?;
        let mut buf = vec![0u8; len as usize];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read Vec<u8>")?;
        Ok(buf)
    }
}

impl Decode for Vec<i32> {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let len = u16::decode(bytes)?;
        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(i32::decode(bytes)?);
        }
        Ok(buf)
    }
}

impl Decode for Vec<f32> {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let len = u16::decode(bytes)?;
        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(f32::decode(bytes)?);
        }
        Ok(buf)
    }
}

impl Decode for Vec<String> {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let len = u16::decode(bytes)?;
        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(String::decode(bytes)?);
        }
        Ok(buf)
    }
}

impl Decode for Vec<Vec<u8>> {
    fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let len = u16::decode(bytes)?;
        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(Vec::<u8>::decode(bytes)?);
        }
        Ok(buf)
    }
}

impl Decode for VarInt {
    fn decode<T>(bytes: &mut T) -> Result<VarInt, Box<(dyn std::error::Error + 'static)>>
    where
        T: Read,
    {
        let mut packet_manager = PacketManager::new(BytesMut::new(), 0);
        Ok(*Box::from(packet_manager.read_var_int(bytes)?))
    }
}
