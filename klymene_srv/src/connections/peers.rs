use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::connections::coordinator::Event;

const BINDING_ADDRESS: &'static str = "127.0.0.1:2242";
pub enum PeerCommand {
    Send(u64, Vec<u8>),
}

pub async fn run(mut cmd_rx: mpsc::Receiver<PeerCommand>, event_tx: mpsc::Sender<Event>) {
    let mut peers: HashMap<u64, mpsc::Sender<Vec<u8>>> = HashMap::new();

    // TODO: listen for an event that creates a connection to a new peer. if that event comes, spawn a thread for it
    loop {
        match cmd_rx.recv().await {
            Some(PeerCommand::Send(id, data)) => {
                if let Some(tx) = peers.get(&id) {
                    let _ = tx.send(data).await;
                }
            }
            None => break,
        }
    }
}
