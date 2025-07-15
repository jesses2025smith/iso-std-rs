use crate::error::Error;
use std::pin::Pin;
use stream_cancel::Valved;
use tokio::sync::mpsc::Sender;
use tokio_stream::Stream;

#[async_trait::async_trait]
pub trait IsoTp {
    type Frame: Clone + Send + 'static;
    /// Get Frame transmitter
    fn transmitter(&self) -> Sender<Self::Frame>;
    /// Get Frame Stream that does not belong to IsoTP
    async fn frame_stream(
        &self,
    ) -> Result<Valved<Pin<Box<dyn Stream<Item = Self::Frame> + Send>>>, Error>;
    /// Start transmit and receive loop worker
    async fn start(&mut self, interval_us: u64);
    /// Stop transmit and receive loop worker
    async fn stop(&mut self);
    /// Wait IsoTP data
    async fn wait_data(&self, timeout: u64) -> Result<bytes::Bytes, Error>;
}
