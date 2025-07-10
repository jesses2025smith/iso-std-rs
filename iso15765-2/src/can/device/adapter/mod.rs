use rs_can::{CanDevice, CanFrame, CanListener};
use std::{collections::HashMap, fmt::Display, sync::Arc, time::{Duration, Instant}};
use tokio::{
    sync::{
        broadcast,
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    task::{spawn, JoinHandle},
    time::sleep,
};

type Listeners<C, F> = Arc<Mutex<HashMap<String, Box<dyn CanListener<C, F>>>>>;
const DEFAULT_STOP_DELAY: u64 = 500;

#[derive(Clone)]
pub struct CanAdapter<D, C, F> {
    pub(crate) device: D,
    pub(crate) sender: Sender<F>,
    pub(crate) receiver: Arc<Mutex<Receiver<F>>>,
    pub(crate) listeners: Listeners<C, F>,
    pub(crate) stop_tx: broadcast::Sender<()>,
    pub(crate) send_task: Arc<Option<JoinHandle<()>>>,
    pub(crate) receive_task: Arc<Option<JoinHandle<()>>>,
    pub(crate) interval: Option<u64>,
}

impl<D, C, F> CanAdapter<D, C, F>
where
    D: CanDevice<Channel = C, Frame = F> + Clone + Send + Sync + 'static,
    C: Clone + Display + Send + Sync + 'static,
    F: CanFrame<Channel = C> + Clone + Display + Send + Sync + 'static,
{
    pub fn new(device: D) -> Self {
        let (tx, rx) = channel(10240);
        let (stop_tx, _) = broadcast::channel(16);
        Self {
            device,
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
            listeners: Arc::new(Mutex::new(HashMap::new())),
            stop_tx,
            send_task: Default::default(),
            receive_task: Default::default(),
            interval: Default::default(),
        }
    }

    #[inline]
    pub async fn register_listener(&self, name: String, listener: Box<dyn CanListener<C, F>>) {
        rsutil::trace!("ISO-TP - register listener {}", name);
        self.listeners.lock().await.insert(name, listener);
    }

    #[inline]
    pub async fn unregister_listener(&self, name: &str) {
        rsutil::trace!("ISO-TP - unregister listener {}", name);
        self.listeners.lock().await.remove(name);
    }

    #[inline]
    pub async fn unregister_all_listeners(&self) {
        self.listeners.lock().await.clear();
    }

    #[inline]
    pub async fn listener_names(&self) -> Vec<String> {
        self.listeners.lock().await.keys().cloned().collect()
    }

    #[inline]
    pub async fn listener_callback(
        &self,
        name: &str,
        callback: impl FnOnce(&Box<dyn CanListener<C, F>>),
    ) {
        if let Some(listener) = self.listeners.lock().await.get(name) {
            callback(listener);
        }
    }

    #[inline]
    pub fn sender(&self) -> Sender<F> {
        self.sender.clone()
    }

    pub async fn start(&mut self, interval_us: u64) {
        self.interval = Some(interval_us);

        let stop_rx = self.stop_tx.subscribe();
        let tx_task = Self::transmit_task(
            self.device.clone(),
            self.receiver.clone(),
            self.listeners.clone(),
            stop_rx,
            interval_us,
        )
        .await;

        let stop_rx = self.stop_tx.subscribe();
        let rx_task = Self::receive_task(
            self.device.clone(),
            self.listeners.clone(),
            stop_rx,
            interval_us,
        )
        .await;

        self.send_task = Arc::new(Some(tx_task));
        self.receive_task = Arc::new(Some(rx_task));
    }

    pub async fn stop(&mut self) {
        rsutil::debug!("ISO-TP - stopping adapter");

        if let Err(e) = self.stop_tx.send(()) {
            rsutil::warn!("ISO-TP - error {} when sending stop signal", e);
        }

        let timeout = Duration::from_millis(DEFAULT_STOP_DELAY);
        let start_time = Instant::now();
        let mut send_task_finished = false;
        let mut receive_task_finished = false;

        while start_time.elapsed() < timeout  {
            send_task_finished = if let Some(task) = &*self.send_task {
                task.is_finished()
            } else {
                true
            };

            receive_task_finished = if let Some(task) = &*self.receive_task {
                task.is_finished()
            } else {
                true
            };

            if send_task_finished && receive_task_finished {
                rsutil::info!("ISO-TP - all tasks stopped successfully");
                break;
            }

            sleep(Duration::from_millis(10)).await;
        }

        if !send_task_finished {
            rsutil::warn!("ISO-TP - transmit task is still running after stop signal");
        }

        if !receive_task_finished {
            rsutil::warn!("ISO-TP - receive task is still running after stop signal");
        }

        self.send_task = Arc::new(None);
        self.receive_task = Arc::new(None);

        self.device.shutdown();
    }

    async fn transmit_task(
        device: D,
        receiver: Arc<Mutex<Receiver<F>>>,
        listeners: Listeners<C, F>,
        mut stop_rx: broadcast::Receiver<()>,
        interval: u64,
    ) -> JoinHandle<()> {
        spawn(async move {
            let device = device.clone();
            let receiver = receiver.clone();
            loop {
                if device.is_closed() {
                    rsutil::info!("ISO-TP - device closed");
                    break;
                }

                if let Some(msg) = receiver.lock().await.recv().await {
                    rsutil::trace!("ISO-TP - transmitting: {}", msg);
                    let id = msg.id();
                    let chl = msg.channel();
                    for listener in listeners.lock().await.values() {
                        listener.on_frame_transmitting(chl.clone(), &msg).await;
                    }
                    if let Ok(_) = device.transmit(msg, Some(100)).await {
                        for listener in listeners.lock().await.values() {
                            listener.on_frame_transmitted(chl.clone(), id).await;
                        }
                    }
                }

                if let Ok(()) = stop_rx.recv().await {
                    rsutil::trace!("ISO-TP - transmit task stopped");
                    break;
                }

                sleep(Duration::from_micros(interval)).await;

                // tokio::select! {
                //     _ = stop_rx.recv() => {
                //         rsutil::trace!("ISO-TP - transmit task stopped");
                //         break;
                //     }
                //     _ = async {
                //         if let Some(msg) = receiver.lock().await.recv().await {
                //             rsutil::trace!("ISO-TP - transmitting: {}", msg);
                //             let id = msg.id();
                //             let chl = msg.channel();
                //             for listener in listeners.lock().await.values() {
                //                 listener.on_frame_transmitting(chl.clone(), &msg).await;
                //             }
                //             if let Ok(_) = device.transmit(msg, Some(100)).await {
                //                 for listener in listeners.lock().await.values() {
                //                     listener.on_frame_transmitted(chl.clone(), id).await;
                //                 }
                //             }
                //         }
                //     } => {},
                //     _ = sleep(Duration::from_micros(interval)) => {}
                // }
            }
        })
    }

    async fn receive_task(
        device: D,
        listeners: Listeners<C, F>,
        mut stop_rx: broadcast::Receiver<()>,
        interval: u64,
    ) -> JoinHandle<()> {
        spawn(async move {
            let device = device.clone();
            loop {
                if device.is_closed() {
                    rsutil::info!("ISO-TP - device closed");
                    break;
                }

                let channels = device.opened_channels();
                for chl in channels {
                    if let Ok(messages) = device.receive(chl.clone(), Some(100)).await {
                        if !messages.is_empty() {
                            for listener in listeners.lock().await.values() {
                                // messages.iter().for_each(|f| rsutil::trace!("ISO-TP - received: {}", f));
                                listener.on_frame_received(chl.clone(), &messages).await;
                            }
                        }
                    }
                }

                if let Ok(()) = stop_rx.recv().await {
                    rsutil::trace!("ISO-TP - receive task stopped");
                    break;
                }

                sleep(Duration::from_micros(interval)).await;

                // tokio::select! {
                //     _ = stop_rx.recv() => {
                //         rsutil::trace!("ISO-TP - receive task stopped");
                //         break;
                //     },
                //     _ = async {
                //         let channels = device.opened_channels();
                //         for chl in channels {
                //             if let Ok(messages) = device.receive(chl.clone(), Some(100)).await {
                //                 if !messages.is_empty() {
                //                     for listener in listeners.lock().await.values() {
                //                         listener.on_frame_received(chl.clone(), &messages).await;
                //                     }
                //                 }
                //             }
                //         }
                //     } => {},
                //     _ = sleep(Duration::from_micros(interval)) => {}
                // }
            }
        })
    }
}
