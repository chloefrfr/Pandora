use bytes::BytesMut;
use packet_manager::{types::varint_types::VarInt, PacketManager};
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub trait Encode {
    #[allow(async_fn_in_trait)]
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin;
}

impl Encode for bool {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin,
    {
        let buf = if *self { [1u8] } else { [0u8] };
        bytes.write_all(&buf).await.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to write bool")
        })
    }
}

impl Encode for u8 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin,
    {
        bytes
            .write_all(&[*self])
            .await
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to write u8"))
    }
}

impl Encode for u16 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin,
    {
        bytes.write_all(&self.to_be_bytes()).await.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to write u16")
        })
    }
}

impl Encode for u32 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin,
    {
        bytes.write_all(&self.to_be_bytes()).await.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to write u32")
        })
    }
}

impl Encode for i32 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin,
    {
        bytes.write_all(&self.to_be_bytes()).await.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to write i32")
        })
    }
}

impl Encode for i64 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin,
    {
        bytes.write_all(&self.to_be_bytes()).await.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to write i64")
        })
    }
}

impl Encode for f32 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin,
    {
        bytes.write_all(&self.to_be_bytes()).await.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to write f32")
        })
    }
}

impl Encode for f64 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin,
    {
        bytes.write_all(&self.to_be_bytes()).await.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to write f64")
        })
    }
}

impl Encode for String {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin,
    {
        let len = VarInt::new(self.len() as i32);
        len.encode(bytes).await?;
        bytes.write_all(&self.as_bytes()).await.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to write String")
        })
    }
}

impl Encode for VarInt {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin,
    {
        let buffer = Vec::new();
        let mut packet_manager = PacketManager::new(BytesMut::from(&buffer[..]), 0);
        packet_manager.write_var_int_checked(self.value);

        bytes.write_all(&buffer).await?;

        Ok(())
    }
}
