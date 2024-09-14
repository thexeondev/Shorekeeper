use std::{net::SocketAddr, sync::Arc};

use shorekeeper_database::PgPool;
use shorekeeper_protokey::ServerProtoKeyHelper;
use tokio::net::UdpSocket;

use crate::{config::NetworkSettings, session::Session, session::SessionManager};

pub struct UdpServer {
    socket: Arc<UdpSocket>,
    protokey_helper: &'static ServerProtoKeyHelper,
    session_mgr: &'static SessionManager,
    db: Arc<PgPool>,
}

impl UdpServer {
    const MTU: usize = 1000;
    const CMD_SYN: u8 = 0xED;
    const CMD_ACK: u8 = 0xEE;

    pub async fn new(
        network_settings: &'static NetworkSettings,
        protokey_helper: &'static ServerProtoKeyHelper,
        session_mgr: &'static SessionManager,
        db: Arc<PgPool>,
    ) -> Result<Self, tokio::io::Error> {
        let socket = UdpSocket::bind(&format!("0.0.0.0:{}", network_settings.kcp_port)).await?;

        Ok(Self {
            socket: Arc::new(socket),
            protokey_helper,
            session_mgr,
            db,
        })
    }

    pub async fn serve(&self) {
        let mut kcp_conv_cnt = 0;
        let mut recv_buf = [0u8; Self::MTU];

        loop {
            let Ok((len, addr)) = self
                .socket
                .recv_from(&mut recv_buf)
                .await
                .inspect_err(|err| tracing::debug!("recv_from failed: {err}"))
            else {
                continue;
            };

            match len {
                1 if recv_buf[0] == Self::CMD_SYN => {
                    kcp_conv_cnt += 1;
                    self.create_session(kcp_conv_cnt, addr).await
                }
                20.. => self.handle_packet(&recv_buf[..len]).await,
                _ => (),
            }
        }
    }

    pub async fn create_session(&self, conv_id: u32, addr: SocketAddr) {
        let session = Session::new(
            conv_id,
            addr,
            self.socket.clone(),
            self.protokey_helper,
            self.db.clone(),
        );
        self.session_mgr.add(conv_id, session);

        let mut ack = Vec::with_capacity(5);
        ack.push(Self::CMD_ACK);
        ack.extend(conv_id.to_le_bytes());
        let _ = self.socket.send_to(&ack, addr).await;

        tracing::debug!("new connection from {addr}, conv_id: {conv_id}");
    }

    pub async fn handle_packet(&self, buf: &[u8]) {
        let conv_id = kcp::get_conv(buf);
        let Some(mut session) = self.session_mgr.get_mut(conv_id) else {
            tracing::debug!("received kcp packet: session with conv_id={conv_id} doesn't exist");
            return;
        };

        let _ = session
            .value_mut()
            .on_receive(buf)
            .await
            .inspect_err(|err| tracing::error!("Session::on_receive failed, error: {err}"));
    }
}
