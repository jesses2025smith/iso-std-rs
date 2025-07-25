use crate::{
    can::isotp::CanIsoTp,
    core::{Event, EventListener},
    error::Error,
    isotp::IsoTp,
};
use bytes::Bytes;
use rs_can::{CanDevice, CanFrame};
use std::{
    fmt::Display,
    pin::Pin,
    time::{Duration, Instant},
};
use stream_cancel::Valved;
use tokio::{sync::mpsc::Sender, time::sleep};
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};

#[async_trait::async_trait]
impl<D, C, F> IsoTp for CanIsoTp<D, C, F>
where
    D: CanDevice<Channel = C, Frame = F> + Clone + Send + 'static,
    C: Clone + Eq + Display + Send + Sync + 'static,
    F: CanFrame<Channel = C> + Clone + Display + 'static,
{
    type Frame = F;

    #[inline(always)]
    fn transmitter(&self) -> Sender<Self::Frame> {
        self.adapter.transmitter()
    }

    #[inline(always)]
    fn shutdown(&mut self) {
        self.adapter.shutdown();
    }

    async fn frame_stream(
        &self,
    ) -> Result<Valved<Pin<Box<dyn Stream<Item = Self::Frame> + Send>>>, Error> {
        let subscriber = self.sender.subscribe();
        let stream: Pin<Box<dyn Stream<Item = Self::Frame> + Send>> =
            Box::pin(BroadcastStream::new(subscriber).filter_map(|v| match v {
                Ok(val) => Some(val),
                Err(e) => {
                    rsutil::warn!("ISO-TP - Error: {} when broadcast non-IsoTP frame", e);
                    None
                }
            }));
        let (trigger, stream) = Valved::new(stream);
        self.triggers.write().await.push(trigger);

        Ok(stream)
    }

    #[inline(always)]
    async fn start(&mut self, interval_us: u64) {
        self.adapter.start(interval_us).await;
    }

    #[inline(always)]
    async fn stop(&mut self) {
        self.adapter.stop().await;
    }

    async fn wait_data(&self, timeout: u64) -> Result<Bytes, Error> {
        let duration = Duration::from_millis(timeout);
        let mut start = Instant::now();

        loop {
            sleep(Duration::from_millis(1)).await;

            if start.elapsed() > duration {
                self.context.clear_buffer().await;
                return Err(Error::Timeout {
                    value: timeout,
                    unit: "ms",
                });
            }

            match self.context.buffer_data().await {
                Some(event) => match event {
                    Event::Wait | Event::FirstFrameReceived => {
                        start = Instant::now();
                    }
                    Event::DataReceived(data) => {
                        // rsutil::trace!("ISO-TP - data received: {}", hex::encode(&data));
                        return Ok(data);
                    }
                    Event::ErrorOccurred(e) => {
                        self.context.clear_buffer().await;
                        return Err(e.clone());
                    }
                },
                None => continue,
            }
        }
    }
}
