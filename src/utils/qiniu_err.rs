#[derive(Debug, Clone)]
pub struct QiniuErr {
    pub message: String,
    pub code: QiniuErrCode,
}

#[derive(Debug, Clone)]
pub enum QiniuErrCode {
    Inval, // Invalid argument
}
