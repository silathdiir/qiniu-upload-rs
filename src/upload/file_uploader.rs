use crate::utils::qiniu_err::QiniuErr;

pub struct FileUploader {}

impl FileUploader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn upload(&self, _key: &str, _file_path: &str) -> Result<(), QiniuErr> {
        Ok(())
    }
}
