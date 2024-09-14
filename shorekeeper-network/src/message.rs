use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::io::{self, Read, Write};

pub struct ServiceMessage {
    pub src_service_id: u32,
    pub rpc_id: u16,
    pub message_id: u16,
    pub data: Box<[u8]>,
}

impl ServiceMessage {
    pub fn encode(&self, buf: &mut [u8]) -> io::Result<()> {
        let mut w = io::Cursor::new(buf);
        w.write_u32::<LE>(self.src_service_id)?;
        w.write_u16::<LE>(self.rpc_id)?;
        w.write_u16::<LE>(self.message_id)?;
        w.write_u32::<LE>(self.data.len() as u32)?;
        w.write_all(&self.data)?;

        Ok(())
    }

    pub fn decode(buf: &[u8]) -> io::Result<Self> {
        let mut r = io::Cursor::new(buf);
        let src_service_id = r.read_u32::<LE>()?;
        let rpc_id = r.read_u16::<LE>()?;
        let message_id = r.read_u16::<LE>()?;
        let data_len = r.read_u32::<LE>()? as usize;

        let mut data = vec![0u8; data_len];
        r.read_exact(&mut data)?;

        Ok(Self {
            src_service_id,
            rpc_id,
            message_id,
            data: data.into_boxed_slice(),
        })
    }

    pub fn get_encoding_length(&self) -> usize {
        12 + self.data.len()
    }
}
