use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub struct ProtocolClient {
    writer: OwnedWriteHalf,
}
