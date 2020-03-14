use thiserror::Error;

#[derive(Debug, Error)]
pub enum QiniuErr {
    #[error("")]
    BadResponse(String),
    #[error("")]
    IOError(std::io::Error),
    #[error("")]
    Inval(String),
    #[error("")]
    Unknown,
}

impl From<std::io::Error> for QiniuErr {
    fn from(err: std::io::Error) -> Self {
        QiniuErr::IOError(err)
    }
}
