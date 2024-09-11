use axum::{extract::State, response::IntoResponse, response::Response};

use crate::config::AesSettings;

pub async fn encrypt_response(
    State(settings): State<&'static AesSettings>,
    rsp: Response,
) -> impl IntoResponse {
    let data = axum::body::to_bytes(rsp.into_body(), usize::MAX)
        .await
        .unwrap();

    crate::util::encrypt_with_aes(&data, settings)
}
