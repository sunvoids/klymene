use tokio::sync::mpsc;

use crate::connections::{peers::PeerCommand, server::ServerCommand};

pub enum Event {
    UnixMessage(String),
    PeerMessage(u64, Vec<u8>),
    ServerMessage(Vec<u8>),
}

pub async fn run(
    mut event_rx: mpsc::Receiver<Event>,
    peer_cmd_tx: mpsc::Sender<PeerCommand>,
    server_cmd_tx: mpsc::Sender<ServerCommand>,
) {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    server_cmd_tx.send(ServerCommand::Login).await.unwrap();
    while let Some(event) = event_rx.recv().await {
        match event {
            Event::UnixMessage(msg) => {
                println!("Received message on Unix socket: {msg}");
            }
            Event::PeerMessage(id, msg) => {
                println!("Peer {id}: {:?}", msg);
            }
            Event::ServerMessage(msg) => {
                println!("Server: {:?}", msg);
            }
        }
    }
}
