use crate::upload;
use crate::utils::auth::Auth;
use crate::utils::qiniu_err::QiniuErr;
use crate::utils::verification::crc32;

use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;

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
        data: &'static [u8],
    ) -> Result<(), QiniuErr> {
        let form = self.build_form(bucket, key, expired_seconds, data)?;
        self.upload_form(form)
    }

    fn build_form(
        &self,
        bucket: &str,
        key: &str,
        expired_seconds: u64,
        data: &'static [u8],
    ) -> Result<Form, QiniuErr> {
        let token = self.auth.upload_token(bucket, key, expired_seconds);
        let crc = crc32(data);
        let file_part = self.build_form_file_part(key, upload::UPLOAD_MIME_TYPE, data)?;

        let form = Form::new()
            .text("token", token)
            .text("key", key.to_string())
            .text("ctc32", crc.to_string())
            .part("file", file_part);

        Ok(form)
    }

    fn build_form_file_part(
        &self,
        filename: &str,
        mime_type: &str,
        data: &'static [u8],
    ) -> Result<Part, QiniuErr> {
        let result = Part::bytes(data)
            .file_name(filename.to_string())
            .mime_str(mime_type);
        match result {
            Ok(file_part) => Ok(file_part),
            Err(err) => Err(QiniuErr::Inval(err.to_string())),
        }
    }

    fn upload_form(&self, form: Form) -> Result<(), QiniuErr> {
        let result = Client::new()
            .post(upload::UPLOAD_URL)
            .multipart(form)
            .send();
        if let Err(err) = result {
            return Err(QiniuErr::Inval(err.to_string()));
        }

        let status_code = result.unwrap().status();
        if status_code.is_success() {
            Ok(())
        } else if status_code.is_server_error() {
            let message = status_code.as_str().to_string();
            Err(QiniuErr::BadResponse(message))
        } else {
            Err(QiniuErr::Unknown)
        }
    }
}
