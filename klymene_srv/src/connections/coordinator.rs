use tokio::sync::mpsc;

use crate::{
    connections::{peers::PeerCommand, server::ServerCommand},
    soulseek::coders::{Login, ProtocolMessage},
};

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
    let tmp = crate::soulseek::coders::LoginRequest {
        username: "username".into(),
        password: "password".into(),
        hash: "d51c9a7e9353746a6020f9602d452929".into(),
        version_number: 160,
        minor_version: 1,
    };
    let tmp_login = Login::encode_request(&tmp);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    server_cmd_tx
        .send(ServerCommand::Send(tmp_login))
        .await
        .unwrap();
    while let Some(event) = event_rx.recv().await {
        match event {
            Event::UnixMessage(msg) => {
                println!("Received message on Unix socket: {msg}");
            }
            Event::PeerMessage(id, msg) => {
                println!("Peer {id}: {:?}", msg);
            }
            Event::ServerMessage(msg) => {
                println!("Server: {:?}", msg); // TODO: Decode into Frame
            }
        }
    }
}
