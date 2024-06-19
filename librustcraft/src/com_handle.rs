use tokio::sync::{mpsc, Mutex, oneshot};
use std::sync::Arc;

pub type RawCCRequestI = String;
pub type RawCCRequestO = String;

pub type ComputerID = String;

pub(crate) struct RawCCRequest {
    pub(crate) tx: oneshot::Sender<RawCCRequestO>,
    pub(crate) data: RawCCRequestI,
}

#[derive(Clone)]
pub struct ComputerHandle {
    sender: mpsc::Sender<RawCCRequest>,
    pub(crate) receiver: Arc<Mutex<mpsc::Receiver<RawCCRequest>>>,
    pub id: ComputerID
}

impl ComputerHandle {
    pub async fn send(&self, data: RawCCRequestI) -> anyhow::Result<RawCCRequestO> {
        let (tx, rx) = oneshot::channel();
        let request = RawCCRequest { tx, data };
        self.sender.send(request).await?;
        Ok(rx.await?)
    }

    pub fn new(id: ComputerID) -> ComputerHandle {
        let (sender, receiver) = mpsc::channel(50);
        ComputerHandle {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            id
        }
    }
}

