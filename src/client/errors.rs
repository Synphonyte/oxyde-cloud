use thiserror::Error;

#[derive(Debug, Error)]
pub enum UploadFileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ReqwestJsonError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}