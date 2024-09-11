use std::collections::HashSet;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProtoKeySettings {
    pub builtin_encryption_msg_id: HashSet<u16>,
    pub use_client_key: bool,
}
