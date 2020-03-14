pub mod data_uploader;
pub mod file_uploader;

use crate::utils::{auth::Auth, qiniu_err::QiniuErr, verification::crc32};

use reqwest::blocking::{
    multipart::{Form, Part},
    Client,
};

use std::{io::Cursor, sync::Arc};

const UPLOAD_MIME_TYPE: &str = "application/octet-stream";
const UPLOAD_URL: &str = "https://upload.qiniup.com";
// const UPLOAD_URL: &str = "https://up-z2.qiniup.com";

fn build_form(
    auth: &Auth,
    bucket: &str,
    key: &str,
    expired_seconds: u64,
    data: Arc<Vec<u8>>,
) -> Result<Form, QiniuErr> {
    let token = auth.upload_token(bucket, key, expired_seconds);
    let crc = crc32(&data);
    let file_part = build_form_file_part(key, UPLOAD_MIME_TYPE, data)?;

    let form = Form::new()
        .text("token", token)
        .text("key", key.to_string())
        .text("ctc32", crc.to_string())
        .part("file", file_part);

    Ok(form)
}

fn build_form_file_part(
    filename: &str,
    mime_type: &str,
    data: Arc<Vec<u8>>,
) -> Result<Part, QiniuErr> {
    let shared_buf = SharedBuf(data);

    let result = Part::reader(Cursor::new(shared_buf))
        .file_name(filename.to_string())
        .mime_str(mime_type);
    match result {
        Ok(file_part) => Ok(file_part),
        Err(err) => Err(QiniuErr::Inval(err.to_string())),
    }
}

fn upload_form(form: Form) -> Result<(), QiniuErr> {
    let result = Client::new().post(UPLOAD_URL).multipart(form).send();
    if let Err(err) = result {
        return Err(QiniuErr::Inval(err.to_string()));
    }

    // println!("Response: {:?}", result.unwrap().text().unwrap());

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

#[derive(Clone)]
struct SharedBuf(Arc<Vec<u8>>);

impl AsRef<[u8]> for SharedBuf {
    fn as_ref(&self) -> &[u8] {
        &**self.0
    }
}
