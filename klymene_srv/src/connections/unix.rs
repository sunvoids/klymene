use tokio::sync::mpsc;

use crate::connections::coordinator::Event;

pub enum UnixCommand {}

pub async fn run(event_tx: mpsc::Sender<Event>) {
    todo!();
}
