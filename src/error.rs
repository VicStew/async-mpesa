use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum MpesaError {
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{}: {}", .0.requestId, .0.errorMessage)]
    ApiError(ApiError),
    #[error("failed to deserialize api response: {0}")]
    JsonDeserialize(serde_json::Error),
    #[error("Invalid args: {0}")]
    InvalidArgument(String),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub errorMessage: String,
    pub requestId: String,
    pub errorCode: String
}

pub(crate) fn map_deserialization_error(e: serde_json::Error, bytes: &[u8]) -> MpesaError {
    tracing::error!(
        "failed deserialization of: {}",
        String::from_utf8_lossy(bytes.as_ref())
    );
    MpesaError::JsonDeserialize(e)
}