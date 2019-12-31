#[derive(Debug, Clone)]
pub struct QiniuErr {
    pub message: String,
    pub code: QiniuErrCode,
}

#[derive(Debug, Clone)]
pub enum QiniuErrCode {
    BadResponse,    // Bad Response
    Inval,          // Invalid argument
    Unknown,        // Unknown error
}
