use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::io::{self, Read, Write};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid message type: {0}")]
    InvalidMessageType(u8),
    #[error("Checksum mismatch, received: {0}, calculated: {1}")]
    InvalidChecksum(u32, u32),
}

#[derive(Debug)]
pub enum Message {
    Request {
        sequence_number: u32,
        rpc_id: u16,
        message_id: u16,
        payload: Option<Box<[u8]>>,
    },
    Response {
        sequence_number: u32,
        rpc_id: u16,
        message_id: u16,
        payload: Option<Box<[u8]>>,
    },
    Push {
        sequence_number: u32,
        message_id: u16,
        payload: Option<Box<[u8]>>,
    },
}

impl Message {
    const TYPE_REQUEST: u8 = 1;
    const TYPE_RESPONSE: u8 = 2;
    const TYPE_PUSH: u8 = 4;

    pub fn encode(&self, out: &mut [u8]) -> io::Result<()> {
        let mut w = io::Cursor::new(out);

        let (sequence_number, message_id, payload) = match self {
            Self::Request {
                sequence_number,
                message_id,
                payload,
                ..
            }
            | Self::Response {
                sequence_number,
                message_id,
                payload,
                ..
            }
            | Self::Push {
                sequence_number,
                message_id,
                payload,
            } => (sequence_number, message_id, payload),
        };

        w.write_u8(self.get_message_type())?;
        w.write_u32::<LE>(*sequence_number)?;
        match self {
            Self::Request { rpc_id, .. } | Self::Response { rpc_id, .. } => {
                w.write_u16::<LE>(*rpc_id)?
            }
            _ => (),
        }
        w.write_u16::<LE>(*message_id)?;

        if let Some(payload) = payload.as_ref() {
            w.write_u32::<LE>(crc32fast::hash(payload))?;
            w.write_all(payload)?;
        } else {
            w.write_u32::<LE>(0)?;
        }

        Ok(())
    }

    pub fn decode(src: &[u8]) -> Result<Self, Error> {
        let mut r = io::Cursor::new(src);
        let message_type = r.read_u8()?;
        let sequence_number = r.read_u32::<LE>()?;
        let rpc_id = match message_type {
            Self::TYPE_REQUEST | Self::TYPE_RESPONSE => r.read_u16::<LE>()?,
            _ => 0,
        };
        let message_id = r.read_u16::<LE>()?;
        let recv_crc = r.read_u32::<LE>()?;

        let mut payload = vec![0u8; src.len() - r.position() as usize].into_boxed_slice();
        let _ = r.read(&mut payload)?;

        let calc_crc = crc32fast::hash(&payload);

        (recv_crc == calc_crc)
            .then_some(())
            .ok_or(Error::InvalidChecksum(recv_crc, calc_crc))?;

        let msg = match message_type {
            Self::TYPE_REQUEST => Self::Request {
                sequence_number,
                rpc_id,
                message_id,
                payload: Some(payload),
            },
            Self::TYPE_RESPONSE => Self::Response {
                sequence_number,
                rpc_id,
                message_id,
                payload: Some(payload),
            },
            Self::TYPE_PUSH => Self::Push {
                sequence_number,
                message_id,
                payload: Some(payload),
            },
            _ => return Err(Error::InvalidMessageType(message_type)),
        };

        Ok(msg)
    }

    pub fn get_encoding_length(&self) -> usize {
        match self {
            Self::Request { payload, .. } | Self::Response { payload, .. } => {
                13 + payload.as_ref().map(|p| p.len()).unwrap_or_default()
            }
            Self::Push { payload, .. } => {
                11 + payload.as_ref().map(|p| p.len()).unwrap_or_default()
            }
        }
    }

    pub fn get_message_type(&self) -> u8 {
        match self {
            Self::Request { .. } => Self::TYPE_REQUEST,
            Self::Response { .. } => Self::TYPE_RESPONSE,
            Self::Push { .. } => Self::TYPE_PUSH,
        }
    }

    pub fn is_request(&self) -> bool {
        matches!(self, Self::Request { .. })
    }

    pub fn is_push(&self) -> bool {
        matches!(self, Self::Push { .. })
    }

    pub fn get_message_id(&self) -> u16 {
        match self {
            Self::Request { message_id, .. }
            | Self::Response { message_id, .. }
            | Self::Push { message_id, .. } => *message_id,
        }
    }

    pub fn get_rpc_id(&self) -> u16 {
        match self {
            Self::Request { rpc_id, .. } | Self::Response { rpc_id, .. } => *rpc_id,
            _ => 0,
        }
    }

    pub fn remove_payload(&mut self) -> Box<[u8]> {
        match self {
            Self::Request { payload, .. }
            | Self::Response { payload, .. }
            | Self::Push { payload, .. } => payload.take().unwrap_or_else(|| Box::new([0u8; 0])),
        }
    }

    pub fn set_payload(&mut self, new_payload: Box<[u8]>) {
        match self {
            Self::Request { payload, .. }
            | Self::Response { payload, .. }
            | Self::Push { payload, .. } => *payload = Some(new_payload),
        }
    }

    pub fn get_sequence_number(&self) -> u32 {
        match self {
            Self::Request {
                sequence_number, ..
            }
            | Self::Response {
                sequence_number, ..
            }
            | Self::Push {
                sequence_number, ..
            } => *sequence_number,
        }
    }
}
