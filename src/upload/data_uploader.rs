use crate::utils::qiniu_err::QiniuErr;

pub struct DataUploader {}

impl DataUploader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn upload(&self, key: &str, data: &Vec<u8>) -> Result<(), QiniuErr> {
        Ok(())
    }
}
