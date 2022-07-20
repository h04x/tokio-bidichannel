use tokio::sync::mpsc;

pub struct Endpoint<T, R> {
    tx: mpsc::UnboundedSender<T>,
    rx: mpsc::UnboundedReceiver<R>,
}

pub fn unbounded_bidichannel<T, K>() -> (Endpoint<T, K>, Endpoint<K, T>) {
    let a = mpsc::unbounded_channel::<T>();
    let b = mpsc::unbounded_channel::<K>();
    (Endpoint { tx: a.0, rx: b.1 }, Endpoint { tx: b.0, rx: a.1 })
}

impl<T, R> Endpoint<T, R> {
    pub fn send(&self, message: T) -> Result<(), mpsc::error::SendError<T>> {
        self.tx.send(message)
    }
    pub async fn recv(&mut self) -> Option<R> {
        self.rx.recv().await
    }
}
