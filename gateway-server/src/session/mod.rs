use std::{fmt, net::SocketAddr, sync::Arc, task};

use common::time_util;
use kcp::Kcp;
use shorekeeper_database::PgPool;
use shorekeeper_protocol::{message::Message, ProtocolUnit};
use shorekeeper_protokey::ServerProtoKeyHelper;
use tokio::{io::AsyncWrite, net::UdpSocket};
use util::LengthFieldBasedDecoder;

use crate::handler::client_message_handler;

mod error;
mod manager;
mod util;
pub use error::SessionError;
pub use manager::SessionManager;

pub struct Session {
    kcp: Kcp<SessionOutput>,
    conv_id: u32,
    addr: SocketAddr,
    decoder: LengthFieldBasedDecoder,
    start_time_ms: u64,
    protokey_helper: &'static ServerProtoKeyHelper,
    session_key: Option<[u8; 32]>,
    last_heartbeat_time_ms: u64,
    pub database: Arc<PgPool>,
    pub user_id: Option<String>,
    pub player_id: Option<i32>,
}

struct SessionOutput {
    output_addr: SocketAddr,
    udp_socket: Arc<UdpSocket>,
}

impl Session {
    pub fn new(
        conv_id: u32,
        addr: SocketAddr,
        socket: Arc<UdpSocket>,
        helper: &'static ServerProtoKeyHelper,
        database: Arc<PgPool>,
    ) -> Self {
        let output = SessionOutput {
            output_addr: addr,
            udp_socket: socket,
        };

        let cur_time_ms = time_util::unix_timestamp_ms();
        Self {
            protokey_helper: helper,
            kcp: Kcp::new(conv_id, true, output),
            decoder: LengthFieldBasedDecoder::new(),
            start_time_ms: cur_time_ms,
            last_heartbeat_time_ms: cur_time_ms,
            session_key: None,
            user_id: None,
            player_id: None,
            conv_id,
            addr,
            database,
        }
    }

    pub async fn on_receive(&mut self, buf: &[u8]) -> Result<(), SessionError> {
        self.kcp.input(buf)?;
        self.kcp.async_update(self.time()).await?;
        self.kcp.async_flush().await?;

        if self.kcp.peeksize().is_ok() {
            let mut buf = [0u8; 1500];
            while let Ok(size) = self.kcp.recv(&mut buf) {
                self.decoder.input(&buf[..size]);
            }
        }

        while let Some(mut message) = self.next_message() {
            if let Some(session_key) = self.session_key.as_ref() {
                let payload = message.remove_payload();
                message.set_payload(self.protokey_helper.decrypt(
                    message.get_message_id(),
                    message.get_sequence_number(),
                    session_key,
                    payload,
                )?);
            }

            client_message_handler::push_message(self.conv_id, message).await;
        }

        Ok(())
    }

    pub async fn send_response(
        &mut self,
        response: impl ProtocolUnit,
        rpc_id: u16,
    ) -> Result<(), SessionError> {
        let message = Message::Response {
            sequence_number: 0,
            rpc_id,
            message_id: response.get_message_id(),
            payload: Some(response.encode_to_vec().into_boxed_slice()),
        };

        self.send_message(message).await?;
        Ok(())
    }

    pub async fn send_message(&mut self, mut message: Message) -> Result<(), SessionError> {
        if let Some(session_key) = self.session_key.as_ref() {
            let payload = message.remove_payload();
            message.set_payload(self.protokey_helper.encrypt(
                message.get_message_id(),
                message.get_sequence_number(),
                session_key,
                payload,
            )?);
        }

        let encoding_length = message.get_encoding_length();
        let mut data = vec![0u8; encoding_length + 3];

        data[0] = (encoding_length & 0xFF) as u8;
        data[1] = ((encoding_length >> 8) & 0xFF) as u8;
        data[2] = ((encoding_length >> 16) & 0xFF) as u8;

        message.encode(&mut data[3..])?;
        self.kcp.send(&data)?;
        self.kcp.async_flush().await?;

        Ok(())
    }

    pub fn update_last_heartbeat_time(&mut self) {
        self.last_heartbeat_time_ms = time_util::unix_timestamp_ms();
    }

    pub fn generate_session_key(&mut self) -> Result<Vec<u8>, SessionError> {
        let (session_key, encrypted_key) = self.protokey_helper.generate_session_key()?;
        self.session_key = Some(session_key);

        Ok(encrypted_key.unwrap_or_default())
    }

    pub fn get_conv_id(&self) -> u32 {
        self.conv_id
    }

    fn time(&self) -> u32 {
        (time_util::unix_timestamp_ms() - self.start_time_ms) as u32
    }

    fn next_message(&mut self) -> Option<Message> {
        self.decoder.pop_with(|buf| {
            Message::decode(&buf)
                .inspect_err(|err| {
                    tracing::error!(
                        "failed to decode a message, err: {err}, buf: {}",
                        hex::encode(&buf)
                    )
                })
                .ok()
        })
    }
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "session(conv_id={}, addr={})", self.conv_id, self.addr)
    }
}

impl AsyncWrite for SessionOutput {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, std::io::Error>> {
        self.udp_socket.poll_send_to(cx, buf, self.output_addr)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), std::io::Error>> {
        task::Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), std::io::Error>> {
        task::Poll::Ready(Ok(()))
    }
}
