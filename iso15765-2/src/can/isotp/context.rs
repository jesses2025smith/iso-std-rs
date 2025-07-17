use crate::{
    can::address::Address,
    constants::CONSECUTIVE_SEQUENCE_START,
    core::{Buffer, Consecutive, Event, EventListener, FlowControlContext, State, Timeout},
    error::Error,
    TIMEOUT_AS_ISO15765_2, TIMEOUT_CR_ISO15765_2,
};
use bitflags::Flags;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    sync::{Mutex, RwLock},
    time::sleep,
};

#[derive(Debug, Default, Clone)]
pub(crate) struct Context {
    pub(crate) address: Arc<RwLock<Address>>,
    pub(crate) buffer: Buffer,
    pub(crate) timeout: Timeout, // todo not used
    pub(crate) flow_ctrl: Arc<Mutex<Option<FlowControlContext>>>,
    pub(crate) consecutive: Arc<Mutex<Consecutive>>,
    pub(crate) state: Arc<Mutex<State>>,
}

impl Context {
    const MAX_TIMEOUT_MS: u64 = 5_000;

    pub fn new(address: Address) -> Self {
        Self {
            address: Arc::new(RwLock::new(address)),
            ..Default::default()
        }
    }

    /// reset st_min/consecutive/block_size
    #[inline]
    pub async fn reset(&self) {
        // let mut guard = self.p2_ctx.lock().await;
        // *guard = Default::default();
        self.state_idle().await;
        self.clear_flow_ctrl().await;
        self.clear_consecutive().await;
    }
    #[inline]
    pub async fn clear_flow_ctrl(&self) {
        let mut gurad = self.flow_ctrl.lock().await;
        *gurad = Default::default();
    }
    #[inline]
    pub async fn update_flow_ctrl(&self, ctx: FlowControlContext) {
        let mut gurad = self.flow_ctrl.lock().await;
        *gurad = Some(ctx);
    }
    #[inline]
    pub async fn clear_consecutive(&self) {
        let mut guard = self.consecutive.lock().await;
        guard.sequence = Default::default();
        guard.length = Default::default();
        guard.buffer.clear();
    }
    #[inline]
    pub async fn update_consecutive(&self, length: u32, mut data: Vec<u8>) {
        let mut guard = self.consecutive.lock().await;
        guard.length = Some(length);
        guard.buffer.extend_from_slice(&mut data);
    }

    pub async fn append_consecutive(
        &self,
        sequence: u8,
        mut data: Vec<u8>,
    ) -> Result<Event, Error> {
        let mut guard = self.consecutive.lock().await;
        if guard.length.is_none() {
            return Err(Error::MixFramesError);
        }

        let target = match guard.sequence {
            Some(v) => match v {
                ..=0x0E => v + 1,
                _ => 0,
            },
            None => CONSECUTIVE_SEQUENCE_START,
        };
        guard.sequence = Some(target);
        if sequence != target {
            return Err(Error::InvalidSequence {
                expect: target,
                actual: sequence,
            });
        }

        guard.buffer.extend_from_slice(&mut data);

        let buff_len = guard.buffer.len();
        let target_len = guard.length.unwrap() as usize;
        if buff_len >= target_len {
            guard.buffer.resize(target_len, 0);
            let data = guard.buffer.clone();
            Ok(Event::DataReceived(data.into()))
        } else {
            Ok(Event::Wait)
        }
    }

    #[inline(always)]
    pub async fn state_remove(&self, flags: State) {
        // println!("state remove: {}", flags);
        let after = {
            let mut guard = self.state.lock().await;
            guard.remove(flags);
            *guard
        };
        rsutil::trace!("ISO-TP - current state(state remove): {}", after);
    }

    #[inline(always)]
    pub async fn state_contains(&self, state: State) -> bool {
        self.state.lock().await.contains(state)
    }

    #[inline(always)]
    pub async fn state_idle(&self) {
        self.state.lock().await.clear();
    }

    #[inline(always)]
    pub async fn state_append(&self, flags: State) {
        // println!("state append: {}", flags);
        let after = {
            let mut guard = self.state.lock().await;
            if flags.contains(State::Error) {
                guard.insert(State::Error);
            } else {
                guard.insert(flags);
            }
            *guard
        };
        rsutil::trace!("ISO-TP - current state(state append): {}", after);
    }

    pub async fn write_waiting(&self, index: &mut usize) -> Result<(), Error> {
        {
            // this is not elegant enough
            if let Some(ctx) = &*self.flow_ctrl.lock().await {
                if ctx.block_size != 0 {
                    if (*index + 1) == ctx.block_size as usize {
                        *index = 0;
                        self.state_append(State::WaitFlowCtrl).await;
                    } else {
                        *index += 1;
                    }
                }
                sleep(Duration::from_micros(ctx.st_min as u64)).await;
            }
            // free `flow_ctrl` lock
        }

        tokio::time::timeout(Duration::from_millis(Self::MAX_TIMEOUT_MS), async move {
            let start = Instant::now();
            loop {
                if let Ok(state) = self.state.try_lock() {
                    if state.contains(State::Error) {
                        return Err(Error::DeviceError);
                    }

                    if state.contains(State::Sending) {
                        if start.elapsed() > Duration::from_millis(TIMEOUT_AS_ISO15765_2 as u64) {
                            return Err(Error::Timeout {
                                value: TIMEOUT_AS_ISO15765_2 as u64,
                                unit: "ms",
                            });
                        }
                    } else if state.contains(State::WaitBusy) {
                        let p2_star = self.get_ar().await;
                        if start.elapsed() > Duration::from_millis(p2_star) {
                            return Err(Error::Timeout {
                                value: p2_star,
                                unit: "ms",
                            });
                        }
                    } else if state.contains(State::WaitFlowCtrl) {
                        if start.elapsed() > Duration::from_millis(TIMEOUT_CR_ISO15765_2 as u64) {
                            return Err(Error::Timeout {
                                value: TIMEOUT_CR_ISO15765_2 as u64,
                                unit: "ms",
                            });
                        }
                    } else if *state == State::Idle {
                        return Ok(());
                    }
                } else {
                    // avoid dead loop
                    sleep(Duration::from_millis(1)).await;
                }
            }
        })
        .await
        .map_err(|_| {
            rsutil::warn!("ISO-TP - write timeout");
            Error::Timeout {
                value: Self::MAX_TIMEOUT_MS,
                unit: "ms",
            }
        })?
    }
}

#[async_trait::async_trait]
impl EventListener for Context {
    #[inline(always)]
    async fn buffer_data(&self) -> Option<Event> {
        self.buffer.get().await
    }
    #[inline(always)]
    async fn clear_buffer(&self) {
        self.buffer.clear().await;
    }
    #[inline(always)]
    async fn on_iso_tp_event(&self, event: Event) {
        self.buffer.set(event).await
    }
}

#[allow(unused)]
impl Context {
    #[inline(always)]
    pub async fn set_as(&self, val: u64) {
        let mut guard = self.timeout.n_as.lock().await;
        *guard = val;
    }
    #[inline(always)]
    pub async fn get_as(&self) -> u64 {
        *self.timeout.n_as.lock().await
    }
    #[inline(always)]
    pub async fn set_bs(&self, val: u64) {
        let mut guard = self.timeout.n_bs.lock().await;
        *guard = val;
    }
    #[inline(always)]
    pub async fn get_bs(&self) -> u64 {
        *self.timeout.n_bs.lock().await
    }
    #[inline(always)]
    pub async fn set_cs(&self, val: u64) {
        let mut guard = self.timeout.n_cs.lock().await;
        *guard = val;
    }
    #[inline(always)]
    pub async fn get_cs(&self) -> u64 {
        *self.timeout.n_cs.lock().await
    }
    #[inline(always)]
    pub async fn set_ar(&self, val: u64) {
        let mut guard = self.timeout.n_ar.lock().await;
        *guard = val;
    }
    #[inline(always)]
    pub async fn get_ar(&self) -> u64 {
        *self.timeout.n_ar.lock().await
    }
    #[inline(always)]
    pub async fn set_br(&self, val: u64) {
        let mut guard = self.timeout.n_br.lock().await;
        *guard = val;
    }
    #[inline(always)]
    pub async fn get_br(&self) -> u64 {
        *self.timeout.n_br.lock().await
    }
    #[inline(always)]
    pub async fn set_cr(&self, val: u64) {
        let mut guard = self.timeout.n_cr.lock().await;
        *guard = val;
    }
    #[inline(always)]
    pub async fn get_cr(&self) -> u64 {
        *self.timeout.n_cr.lock().await
    }
}
