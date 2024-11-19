pub trait Encode {
    #[allow(async_fn_in_trait)]
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: tokio::io::AsyncWrite + tokio::io::AsyncSeek + Unpin;
}
