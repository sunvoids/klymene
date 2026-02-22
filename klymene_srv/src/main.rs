use std::time::Duration;

use tokio::{
    sync::{broadcast, oneshot},
    time::sleep,
};

use crate::slsk::client::{peer_connections, server_connection};

mod slsk;

// Main thread:
// - creates and holds handles to server and peer threads,
// - manages channels of these threads and works with their info,
// - handles Unix socket to communicate with klymene_tui,
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // This isn't the best way to do it. Architecturally, the below oneshot channels are non-sensical
    // and will be worked on in the next commit. Since we will have proper channels for communication,
    // it can be derived that when a channel for communication is closed, the threads are expected to
    // shut down as well. This means that threads should `match recv()` in the loop, and when
    // Err(RecvError::Closed) matches (which, per docs, implies all sending halves have been dropped),
    // the thread will be shut down as well.
    let (shutdown_sender, _) = broadcast::channel(1);
    let server_quitter = shutdown_sender.subscribe();
    let peer_quitter = shutdown_sender.subscribe();

    // As explained above, this isn't the way it should work, we won't oneshot Vecs of bytes back
    // and forth. There will be just one mpsc channel, server thread and peer thread will receive
    // tasks, threads will act on them. There may be times when servers send a task back to main,
    // so main will have to coordinate.
    let server_channel = oneshot::channel();
    let peer_channel = oneshot::channel();

    let server_connection_thread = tokio::spawn(server_connection(server_quitter, server_channel));
    let peer_connections_thread = tokio::spawn(peer_connections(peer_quitter, peer_channel));

    // sleep(Duration::from_secs(5)).await;

    shutdown_sender
        .send(())
        .expect("receivers won't be dropped");

    // Don't think these should panic, just log.
    server_connection_thread
        .await
        .expect("awaiting the server thread")
        .expect("server thread returned an error");
    peer_connections_thread
        .await
        .expect("awaiting the peers thread")
        .expect("peers thread returned an error");
    Ok(())
}
