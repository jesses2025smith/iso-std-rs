use rs_can::{CanDevice, CanFrame, CanListener};
use std::{collections::HashMap, fmt::Display, sync::Arc, time::Duration};
use tokio::{
    sync::{
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
    pub(crate) stop_tx: Sender<()>,
    pub(crate) stop_rx: Arc<Mutex<Receiver<()>>>,
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
        let (stop_tx, stop_rx) = channel(10240);
        Self {
            device,
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
            listeners: Arc::new(Mutex::new(HashMap::new())),
            stop_tx,
            stop_rx: Arc::new(Mutex::new(stop_rx)),
            send_task: Default::default(),
            receive_task: Default::default(),
            interval: Default::default(),
        }
    }

    #[inline]
    pub async fn register_listener(&self, name: String, listener: Box<dyn CanListener<C, F>>) {
        rsutil::trace!("SyncISO-TP - register listener {}", name);
        self.listeners.lock().await.insert(name, listener);
    }

    #[inline]
    pub async fn unregister_listener(&self, name: &str) {
        rsutil::trace!("SyncISO-TP - unregister listener {}", name);
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
        let device = Arc::new(Mutex::new(self.device.clone()));

        let tx_task = Self::transmit_task(
            device.clone(),
            self.listeners.clone(),
            self.stop_rx.clone(),
            interval_us,
        )
        .await;

        let rx_task = Self::receive_task(
            device.clone(),
            self.receiver.clone(),
            self.listeners.clone(),
            self.stop_rx.clone(),
            interval_us,
        )
        .await;

        self.send_task = Arc::new(Some(tx_task));
        self.receive_task = Arc::new(Some(rx_task));
    }

    pub async fn stop(&mut self) {
        rsutil::info!("SyncISO-TP - stopping adapter");
        if let Err(e) = self.stop_tx.send(()).await {
            rsutil::warn!("SyncISO-TP - error {} when stopping transmit", e);
        }

        sleep(Duration::from_micros(
            2 * self.interval.unwrap_or(DEFAULT_STOP_DELAY),
        ))
        .await;

        if let Some(task) = &*self.send_task {
            if !task.is_finished() {
                rsutil::warn!("SyncISO-TP - transmit task is running after stop signal");
            }
        }

        if let Some(task) = &*self.receive_task {
            if !task.is_finished() {
                rsutil::warn!("SyncISO-TP - receive task is running after stop signal");
            }
        }

        self.device.shutdown();
    }

    async fn transmit_task(
        device: Arc<Mutex<D>>,
        listeners: Listeners<C, F>,
        stop_rx: Arc<Mutex<Receiver<()>>>,
        interval: u64,
    ) -> JoinHandle<()> {
        spawn(async move {
            let device = device.clone();
            loop {
                if device.lock().await.is_closed() {
                    rsutil::info!("SyncISO-TP - device closed");
                    break;
                }

                let dev_mutex = device.lock().await;
                let channels = dev_mutex.opened_channels();
                for chl in channels {
                    if let Ok(messages) = dev_mutex.receive(chl.clone(), Some(100)).await {
                        if !messages.is_empty() {
                            for listener in listeners.lock().await.values() {
                                listener.on_frame_received(chl.clone(), &messages).await;
                            }
                        }
                    }
                }

                if let Some(()) = stop_rx.lock().await.recv().await {
                    rsutil::info!("SyncISO-TP - stop sync");
                    break;
                }

                sleep(Duration::from_micros(interval)).await;
            }
        })
    }

    async fn receive_task(
        device: Arc<Mutex<D>>,
        receiver: Arc<Mutex<Receiver<F>>>,
        listeners: Listeners<C, F>,
        stop_rx: Arc<Mutex<Receiver<()>>>,
        interval: u64,
    ) -> JoinHandle<()> {
        spawn(async move {
            let device = device.clone();
            let receiver = receiver.clone();
            loop {
                if device.lock().await.is_closed() {
                    rsutil::info!("SyncISO-TP - device closed");
                    break;
                }

                if let Some(msg) = receiver.lock().await.recv().await {
                    rsutil::trace!("SyncISO-TP - transmitting: {}", msg);
                    let id = msg.id();
                    let chl = msg.channel();
                    for listener in listeners.lock().await.values() {
                        listener.on_frame_transmitting(chl.clone(), &msg).await;
                    }

                    if let Ok(_) = device.lock().await.transmit(msg, Some(100)).await {
                        for listener in listeners.lock().await.values() {
                            listener.on_frame_transmitted(chl.clone(), id).await;
                        }
                    }
                }

                if let Some(()) = stop_rx.lock().await.recv().await {
                    rsutil::info!("SyncISO-TP - stop sync");
                    break;
                }

                sleep(Duration::from_micros(interval)).await;
            }
        })
    }
}
