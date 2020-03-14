use crate::upload::{build_form, upload_form};
use crate::utils::{auth::Auth, qiniu_err::QiniuErr};

use std::{
    fs::File,
    io::{BufReader, Read},
    sync::Arc,
};

pub struct FileUploader {
    auth: Auth,
}

impl FileUploader {
    pub fn new(auth: Auth) -> Self {
        Self { auth }
    }

    pub fn upload(
        &self,
        bucket: &str,
        key: &str,
        expired_seconds: u64,
        file_path: &str,
    ) -> Result<(), QiniuErr> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        let form = build_form(&self.auth, bucket, key, expired_seconds, Arc::new(data))?;
        upload_form(form)
    }
}
