use tokio::sync::mpsc;
use zeromq::{PullSocket, Socket, SocketRecv, ZmqError};

use crate::{config::ServiceEndPoint, ServiceMessage};

pub struct ServiceListener {
    receiver: mpsc::Receiver<ServiceMessage>,
}

impl ServiceListener {
    pub async fn bind(end_point: &ServiceEndPoint) -> Result<Self, ZmqError> {
        let mut socket = PullSocket::new();
        socket.bind(&end_point.addr).await?;

        let (tx, rx) = mpsc::channel(32);
        tokio::spawn(async move { Self::recv_task_fn(socket, tx).await });

        Ok(Self { receiver: rx })
    }

    pub async fn receive(&mut self) -> Option<ServiceMessage> {
        self.receiver.recv().await
    }

    async fn recv_task_fn(mut socket: PullSocket, sender: mpsc::Sender<ServiceMessage>) {
        loop {
            let data = socket.recv().await.unwrap();
            for message in data
                .into_vec()
                .into_iter()
                .map(|b| ServiceMessage::decode(b.as_ref()))
                .flatten()
            {
                let _ = sender.send(message).await;
            }
        }
    }
}
