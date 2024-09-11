use crate::util::from_base64;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NetworkSettings {
    pub http_addr: String,
}

#[derive(Deserialize)]
pub struct AesSettings {
    #[serde(deserialize_with = "from_base64")]
    pub key: Vec<u8>,
    #[serde(deserialize_with = "from_base64")]
    pub iv: Vec<u8>,
}
