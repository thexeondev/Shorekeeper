#[derive(thiserror::Error, Debug)]
pub enum SessionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("KCP transport error: {0}")]
    KcpError(#[from] kcp::Error),
    #[error("Crypto error: {0}")]
    CryptoError(#[from] shorekeeper_protokey::Error),
    #[error("Protobuf decode error: {0}")]
    DecodeFailed(#[from] shorekeeper_protocol::ProtobufDecodeError),
}
