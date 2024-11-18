use std::error::Error;

use packet_manager::{types::varint_types::VarInt, PacketManager};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek};

pub trait Decode {
    #[allow(async_fn_in_trait)]
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Box<dyn Error>>
    where
        T: AsyncRead + AsyncSeek + Unpin;
}

impl Decode for bool {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Box<dyn Error>>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 1];
        bytes.read_exact(&mut buf).await.map_err(|_| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to read bool",
            ))
        })?;
        Ok(Box::from(buf[0] != 0))
    }
}

impl Decode for u8 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Box<dyn Error>>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 1];
        bytes.read_exact(&mut buf).await.map_err(|_| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to read u8",
            ))
        })?;
        Ok(Box::from(buf[0]))
    }
}

impl Decode for u16 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Box<dyn Error>>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 2];
        bytes.read_exact(&mut buf).await.map_err(|_| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to read u16",
            ))
        })?;
        Ok(Box::from(u16::from_be_bytes(buf)))
    }
}

impl Decode for u32 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Box<dyn Error>>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 4];
        bytes.read_exact(&mut buf).await.map_err(|_| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to read u32",
            ))
        })?;
        Ok(Box::from(u32::from_be_bytes(buf)))
    }
}

impl Decode for i32 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Box<dyn Error>>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 4];
        bytes.read_exact(&mut buf).await.map_err(|_| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to read i32",
            ))
        })?;
        Ok(Box::from(i32::from_be_bytes(buf)))
    }
}

impl Decode for String {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Box<dyn Error>>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let remaining_bytes: VarInt = PacketManager::read_var_int(bytes).await?;

        let mut string = vec![0u8; remaining_bytes.to_i32() as usize];
        bytes.read_exact(&mut string).await?;
        Ok(Box::from(String::from_utf8(string)?))
    }
}

impl Decode for VarInt {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Box<dyn Error>>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        Ok(Box::from(PacketManager::read_var_int(bytes).await?))
    }
}
