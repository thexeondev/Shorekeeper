mod manager;
pub use manager::SessionManager;
use shorekeeper_network::ServiceMessage;
use shorekeeper_protocol::{message::Message, ForwardClientMessagePush, MessageID, Protobuf};

use crate::{gateway_connection, logic::thread_mgr::LogicThreadHandle};

pub struct Session {
    pub gateway_id: u32,
    pub session_id: u32,
    pub player_id: i32,
    pub logic_thread: LogicThreadHandle,
}

impl Session {
    pub fn forward_to_gateway(&self, message: Message) {
        let mut data = vec![0u8; message.get_encoding_length()];
        message.encode(&mut data).unwrap();

        let push = ForwardClientMessagePush {
            gateway_session_id: self.session_id,
            data,
        };

        gateway_connection::push_message_sync(ServiceMessage {
            src_service_id: 0,
            rpc_id: 0,
            message_id: ForwardClientMessagePush::MESSAGE_ID,
            data: push.encode_to_vec().into_boxed_slice(),
        })
    }

    pub fn get_global_session_id(&self) -> u64 {
        Self::global_id(self.gateway_id, self.session_id)
    }

    #[inline]
    pub fn global_id(gateway_id: u32, session_id: u32) -> u64 {
        ((gateway_id as u64) << 32) | session_id as u64
    }
}
