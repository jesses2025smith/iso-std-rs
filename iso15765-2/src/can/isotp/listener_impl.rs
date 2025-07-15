use crate::{
    can::isotp::CanIsoTp,
    core::{Event, State},
    frame::Frame,
};
use rs_can::{CanDevice, CanFrame, CanId, CanListener};
use std::{any::Any, fmt::Display, sync::Weak};

#[async_trait::async_trait]
impl<D, C, F> CanListener<C, F> for CanIsoTp<D, C, F>
where
    D: CanDevice<Channel = C, Frame = F> + Clone + Send + Sync + 'static,
    C: Clone + Eq + Display + Send + Sync + 'static,
    F: CanFrame<Channel = C> + Clone + Display + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn on_frame_transmitted(&self, channel: C, id: CanId) {
        let id = id.into_bits();
        rsutil::trace!("ISO-TP - transmitted: {:04X} from {}", id, channel);
        if channel != self.channel {
            return;
        }
        let (tx_id, fid) = {
            let guard = self.context.address.read().await;
            (guard.tx_id, guard.fid)
        };
        if id == tx_id || id == fid {
            self.context.state_remove(State::Sending).await;
        }
    }

    async fn on_frame_received(&self, frames: Weak<Vec<F>>) {
        let (tx_id, rx_id) = {
            let guard = self.context.address.read().await;
            (guard.tx_id, guard.rx_id)
        };
        match frames.upgrade() {
            Some(frames) => {
                for frame in frames.iter() {
                    let channel = frame.channel();
                    if channel != self.channel {
                        if let Err(e) = self.sender.send(frame.clone()) {
                            rsutil::warn!("ISO-TP - Error: {} when sending frame that belongs to other channel", e);
                        }
                        continue;
                    }

                    if frame.id().into_bits() != rx_id {
                        if let Err(e) = self.sender.send(frame.clone()) {
                            rsutil::warn!("ISO-TP - Error: {} when sending non-IsoTP frame", e);
                        }
                        continue;
                    }

                    if self.context.state_contains(State::Error).await {
                        break;
                    }

                    rsutil::debug!("ISO-TP - Received: {}", frame);

                    match Frame::decode(frame.data()) {
                        Ok(frame) => match frame {
                            Frame::SingleFrame { data } => {
                                // rsutil::trace!("ISO-TP - received single frame");
                                self.on_single_frame(data).await;
                            }
                            Frame::FirstFrame { length, data } => {
                                // rsutil::trace!("ISO-TP - received first frame");
                                self.on_first_frame(tx_id, length, data).await;
                            }
                            Frame::ConsecutiveFrame { sequence, data } => {
                                // rsutil::trace!("ISO-TP - received consecutive frame");
                                self.on_consecutive_frame(sequence, data).await;
                            }
                            Frame::FlowControlFrame(ctx) => {
                                // rsutil::trace!("ISO-TP - received flow control frame");
                                self.on_flow_ctrl_frame(ctx).await;
                            }
                        },
                        Err(e) => {
                            rsutil::warn!("ISO-TP - data convert to frame failed: {}", e);
                            self.context.state_append(State::Error).await;
                            self.iso_tp_event(Event::ErrorOccurred(e)).await;

                            break;
                        }
                    }
                }
            }
            None => rsutil::warn!("ISO-TP - can't upgrade received frames"),
        }
    }
}
