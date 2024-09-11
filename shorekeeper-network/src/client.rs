use std::time::Duration;

use tokio::sync::mpsc;
use zeromq::{PushSocket, Socket, SocketSend, ZmqMessage};

use crate::{config::ServiceEndPoint, ServiceMessage};

pub struct ServiceClient {
    sender: mpsc::Sender<ServiceMessage>,
}

impl ServiceClient {
    const RECONNECT_ATTEMPT_INTERVAL: Duration = Duration::from_millis(500);

    pub fn new(own_service_id: u32, end_point: &'static ServiceEndPoint) -> Self {
        let (tx, rx) = mpsc::channel(32);
        tokio::spawn(async move { Self::worker_task_fn(own_service_id, end_point, rx).await });

        Self { sender: tx }
    }

    pub async fn push(&self, message: ServiceMessage) {
        let _ = self.sender.send(message).await;
    }

    pub fn push_sync(&self, message: ServiceMessage) {
        let _ = self.sender.blocking_send(message);
    }

    async fn worker_task_fn(
        own_service_id: u32,
        end_point: &'static ServiceEndPoint,
        mut rx: mpsc::Receiver<ServiceMessage>,
    ) {
        let mut socket = None;

        loop {
            let mut message = rx.recv().await.unwrap();
            message.src_service_id = own_service_id;

            let encoding_length = message.get_encoding_length();

            let mut buf = vec![0u8; encoding_length];
            message.encode(&mut buf).unwrap();

            let sock = match socket.as_mut() {
                Some(socket) => socket,
                None => {
                    socket = Some(Self::connect(end_point).await);
                    socket.as_mut().unwrap()
                }
            };

            if let Err(_err) = sock.send(ZmqMessage::from(buf)).await {
                socket = None;
            }
        }
    }

    async fn connect(end_point: &ServiceEndPoint) -> PushSocket {
        let mut new_socket = PushSocket::new();
        loop {
            if new_socket.connect(&end_point.addr).await.is_ok() {
                break new_socket;
            }

            tokio::time::sleep(Self::RECONNECT_ATTEMPT_INTERVAL).await;
        }
    }
}
