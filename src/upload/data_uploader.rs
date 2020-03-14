use crate::{
    upload::{build_form, upload_form},
    utils::{auth::Auth, qiniu_err::QiniuErr},
};

use std::sync::Arc;

pub struct DataUploader {
    auth: Auth,
}

impl DataUploader {
    pub fn new(auth: Auth) -> Self {
        Self { auth }
    }

    pub fn upload(
        &self,
        bucket: &str,
        key: &str,
        expired_seconds: u64,
        data: Arc<Vec<u8>>,
    ) -> Result<(), QiniuErr> {
        let form = build_form(&self.auth, bucket, key, expired_seconds, data)?;
        upload_form(form)
    }
}
