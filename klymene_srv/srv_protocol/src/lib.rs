use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub trait ProtocolMessage: Sized {
    const CODE: u32;

    type Request;
    type Success;
    type Failure;

    fn encode_request(req: &Self::Request) -> Vec<u8>;
    fn decode_success(data: &[u8]) -> Self::Success;
    fn decode_failure(data: &[u8]) -> Self::Failure;
}

pub trait BinaryEncode {
    fn encode(&self, buf: &mut Vec<u8>);
}

pub trait BinaryDecode {
    fn decode(data: &[u8]) -> Self;
}

impl BinaryEncode for u32 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend(self.to_le_bytes());
    }
}

impl BinaryEncode for String {
    fn encode(&self, buf: &mut Vec<u8>) {
        (self.len() as u32).encode(buf);
        buf.extend(self.as_bytes());
    }
}

#[derive(Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub version_number: u32,
    pub hash: String,
    pub minor_version: u32,
}

#[derive(Debug)]
pub struct LoginSuccess {
    greet: String,
    own_ip_address: u32,
    hash: String,
    is_supporter: bool,
}

impl BinaryDecode for LoginSuccess {
    fn decode(data: &[u8]) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum LoginFailure {
    InvalidUsername(UsernameRejectionReason),
    Other,
}

impl BinaryDecode for LoginFailure {
    fn decode(data: &[u8]) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum UsernameRejectionReason {
    Empty,
    TooLong,
    InvalidCharacters,
    LeadingOrTrailingSpaces,
}

impl BinaryEncode for LoginRequest {
    fn encode(&self, buf: &mut Vec<u8>) {
        self.username.encode(buf);
        self.password.encode(buf);
        self.version_number.encode(buf);
        self.hash.encode(buf);
        self.minor_version.encode(buf);
    }
}

#[macro_export]
macro_rules! protocol_message {
    (
        $name:ident,
        code = $code:expr,
        request = $req:ty,
        success = $success:ty,
        failure = $failure:ty
    ) => {
        pub struct $name;

        impl ProtocolMessage for $name {
            const CODE: u32 = $code;

            type Request = $req;
            type Success = $success;
            type Failure = $failure;

            fn encode_request(req: &Self::Request) -> Vec<u8> {
                let mut buf = Vec::new();
                Self::CODE.encode(&mut buf);
                req.encode(&mut buf);
                buf
            }

            fn decode_success(data: &[u8]) -> Self::Success {
                <$success as BinaryDecode>::decode(data)
            }
            fn decode_failure(data: &[u8]) -> Self::Failure {
                <$failure as BinaryDecode>::decode(data)
            }
        }
    };
}

protocol_message!(
    Login,
    code = 0x01,
    request = LoginRequest,
    success = LoginSuccess,
    failure = LoginFailure
);

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
