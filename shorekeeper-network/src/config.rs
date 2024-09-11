use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServiceEndPoint {
    pub addr: String,
}
