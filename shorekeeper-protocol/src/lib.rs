include!("../generated/proto_config.rs");
include!("../generated/data.rs");
include!("../generated/shorekeeper.rs");
include!("../generated/internal.rs");

pub mod message;
pub use prost::DecodeError as ProtobufDecodeError;
pub use prost::Message as Protobuf;

pub trait MessageID {
    const MESSAGE_ID: u16;

    fn get_message_id(&self) -> u16 {
        Self::MESSAGE_ID
    }
}

pub trait ProtocolUnit: Protobuf + MessageID {}
impl<T: Protobuf + MessageID> ProtocolUnit for T {}

#[derive(Debug, PartialEq)]
pub enum MessageRoute {
    None,
    Gateway,
    GameServer,
}

impl From<proto_config::MessageFlags> for MessageRoute {
    fn from(flags: proto_config::MessageFlags) -> Self {
        match flags.value() & 3 {
            0 => Self::Gateway,
            2 => Self::GameServer,
            _ => Self::None,
        }
    }
}
