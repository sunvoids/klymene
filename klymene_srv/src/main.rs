use tokio::sync::mpsc;

use crate::connections::*;

mod connections;
mod slsk;

// Main thread:
// - creates and holds handles to server and peer threads,
// - manages channels of these threads and works with their info,
// - handles Unix socket to communicate with klymene_tui,
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (event_tx, event_rx) = mpsc::channel(128);
    let (peer_cmd_tx, peer_cmd_rx) = mpsc::channel(64);
    let (server_cmd_tx, server_cmd_rx) = mpsc::channel(64);

    tokio::spawn(peers::run(peer_cmd_rx, event_tx.clone()));
    tokio::spawn(server::run(server_cmd_rx, event_tx.clone()));
    // tokio::spawn(unix::run(event_tx.clone()));

    coordinator::run(event_rx, peer_cmd_tx, server_cmd_tx).await;
    Ok(())
}
