use srv_protocol::{Login, LoginRequest};
use tokio::net::TcpStream;
mod slsk;
const SOULFIND_ADDRESS: &'static str = "127.0.0.1:2242";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(SOULFIND_ADDRESS).await?;
    let login_req = LoginRequest {
        username: "username".into(),
        password: "password".into(),
        version_number: 160,
        hash: "d51c9a7e9353746a6020f9602d452929".into(),
        minor_version: 1,
    };
    let _res = Login::send(&login_req, &mut stream).await;
    Ok(())
}
