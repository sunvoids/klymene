use std::time::Duration;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};

use crate::connections::coordinator::Event;

const SOULFIND_ADDRESS: &'static str = "127.0.0.1:2243";
pub enum ServerCommand {
    Send(Vec<u8>),
}

pub async fn run(mut cmd_rx: mpsc::Receiver<ServerCommand>, event_tx: mpsc::Sender<Event>) {
    let mut server_stream = {
        const DELAY_DURATION: u64 = 5;
        let result;
        loop {
            match TcpStream::connect(SOULFIND_ADDRESS).await {
                Ok(v) => {
                    result = v;
                    break;
                }
                Err(e) => {
                    eprintln!(
                        "Failed to connect with Soulseek server ({}). Retrying in {} seconds.",
                        e, DELAY_DURATION
                    );
                    tokio::time::sleep(Duration::from_secs(DELAY_DURATION)).await;
                }
            }
        }
        result
    };
    println!(
        "Established connection with the server: {:?} -> {:?}",
        server_stream.local_addr(),
        server_stream.peer_addr()
    );

    let mut buf = [0u8; 4096];

    loop {
        tokio::select! {
            received = server_stream.read(&mut buf) => {
                if let Ok(n) = received {
                    if n == 0 {
                        break;
                    }
                    let _ = event_tx.send(
                        Event::ServerMessage(buf[..n].to_vec())
                    ).await;
                }
            }

            Some(cmd) = cmd_rx.recv() => {
                match cmd {
                    ServerCommand::Send(data) => {
                        let _ = server_stream.write_all(&data).await;
                    }
                }
            }
        }
    }
}
