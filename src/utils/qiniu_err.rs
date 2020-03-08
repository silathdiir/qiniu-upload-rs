use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum QiniuErr {
    #[error("")]
    BadResponse(String), // Bad Response
    #[error("")]
    Inval(String), // Invalid argument
    #[error("")]
    Unknown, // Unknown error
}
