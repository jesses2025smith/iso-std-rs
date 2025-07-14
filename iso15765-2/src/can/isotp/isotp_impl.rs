use crate::{
    can::{isotp::CanIsoTp, AddressType},
    core::State,
    error::Error,
    frame::Frame,
};
use rs_can::{CanDevice, CanFrame, CanId};
use std::fmt::Display;

impl<D, C: Clone, F: CanFrame<Channel = C> + Display> CanIsoTp<D, C, F>
where
    D: CanDevice<Channel = C, Frame = F> + Clone + Send + Sync + 'static,
    C: Clone + Eq + Display + Send + Sync + 'static,
    F: CanFrame<Channel = C> + Clone + Display + Send + Sync + 'static,
{
    pub async fn transmit<T>(&self, addr_type: AddressType, data: T) -> Result<(), Error>
    where
        T: AsRef<[u8]>,
    {
        self.context.state_idle().await;
        self.context.reset().await;
        rsutil::trace!("ISO-TP - Sending: {}", hex::encode(&data));

        let frames = Frame::from_data(data)?;
        let frame_len = frames.len();

        let (tx_id, fid) = {
            let guard = self.context.address.read().await;
            (guard.tx_id, guard.fid)
        };
        let can_id = match addr_type {
            AddressType::Physical => tx_id,
            AddressType::Functional => fid,
        };
        let mut need_flow_ctrl = frame_len > 1;
        let mut index = 0;
        for iso_tp_frame in frames {
            let data = iso_tp_frame.encode(None);
            let mut frame =
                F::new(CanId::from_bits(can_id, None), data.as_slice()).ok_or_else(|| {
                    rsutil::warn!("fail to convert iso-tp frame to can frame");
                    Error::DeviceError
                })?;
            frame.set_channel(self.channel.clone());

            if need_flow_ctrl {
                need_flow_ctrl = false;
                self.context
                    .state_append(State::Sending | State::WaitFlowCtrl)
                    .await;
            } else {
                self.context.write_waiting(&mut index).await?;
                self.context.state_append(State::Sending).await;
            }
            self.adapter.transmitter.send(frame).await.map_err(|e| {
                rsutil::warn!("ISO-TP - transmit failed: {:?}", e);
                Error::DeviceError
            })?;
        }

        Ok(())
    }
}
