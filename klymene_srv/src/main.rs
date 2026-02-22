use protocol::add;
#[allow(dead_code)] // TEMP
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
const SOULFIND_ADDRESS: &'static str = "127.0.0.1:2242";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("{}", add(1, 2));
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

pub trait ProtocolMessage: Sized {
    const CODE: u32;

    type Request;
    type Success;
    type Failure;

    fn encode_request(req: &Self::Request) -> Vec<u8>;
    fn decode_success(data: &[u8]) -> Self::Success;
    fn decode_failure(data: &[u8]) -> Self::Failure;
}

#[derive(Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
    version_number: u32,
    hash: String,
    minor_version: u32,
}

#[derive(Debug)]
pub struct LoginSuccess {
    greet: String,
    own_ip_address: u32,
    hash: String,
    is_supporter: bool,
}

#[derive(Debug)]
pub struct LoginFailure {
    rejection_reason: String,
}

#[derive(Debug)]
pub struct Login;

impl ProtocolMessage for Login {
    const CODE: u32 = 0x01;

    type Request = LoginRequest;
    type Success = LoginSuccess;
    type Failure = LoginFailure;

    // TODO: macroing this later on
    fn encode_request(req: &Self::Request) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(Self::CODE.to_le_bytes());
        buf.extend((req.username.len() as u32).to_le_bytes());
        buf.extend(req.username.as_bytes());
        buf.extend((req.password.len() as u32).to_le_bytes());
        buf.extend(req.password.as_bytes());
        buf.extend(req.version_number.to_le_bytes());
        buf.extend((req.hash.len() as u32).to_le_bytes());
        buf.extend(req.hash.as_bytes());
        buf.extend(req.minor_version.to_le_bytes());
        buf
    }

    // TODO: not sure if these will be macro-able, as there are conditions for different enums in responses
    fn decode_success(data: &[u8]) -> Self::Success {
        for b in data {
            print!("{:02x}", b);
        }
        todo!()
    }

    fn decode_failure(data: &[u8]) -> Self::Failure {
        for b in data {
            print!("{:02x}", b);
        }
        todo!()
    }
}

impl Login {
    pub async fn send(
        req: &<Login as ProtocolMessage>::Request,
        transport: &mut tokio::net::TcpStream,
    ) -> Result<
        Result<<Login as ProtocolMessage>::Success, <Login as ProtocolMessage>::Failure>,
        std::io::Error,
    > {
        let mut payload = vec![];
        let bytes = Self::encode_request(req);
        println!("{}", bytes.len());
        let length = (bytes.len() as u32).to_le_bytes();
        payload.extend(&length);
        payload.extend(&bytes);
        for b in &payload {
            print!("{:02x} ", b);
        }
        println!("{:?}", req);
        transport.write_all(&payload).await?;
        let mut response_buf = [0u8; 512];
        let response = transport.read(&mut response_buf).await;
        let result = match response {
            Ok(len) => {
                println!("received a response of length {len}");
                let response_slice = &response_buf[..len];
                match response_slice.split_first() {
                    Some((flag, data)) => {
                        if *flag == 0 {
                            Ok(Err(Self::decode_failure(&data)))
                        } else {
                            Ok(Ok(Self::decode_success(&data)))
                        }
                    }
                    None => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "soulseek server returned 0 length response or timed out",
                    )),
                }
            }
            Err(e) => Err(e),
        };

        result
    }
}
