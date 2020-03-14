use qiniu_upload_rs::{upload::data_uploader::DataUploader, utils::auth::Auth};
use std::sync::Arc;

const ACCESS_KEY: &str = "test-access-key";
const SECRET_KEY: &str = "test-secret-key";

const BUCKET: &str = "test-bucket";
const KEY: &str = "test-data-key.txt";
const EXPIRED_SECONDS: u64 = 3600;

const DATA: &str = "hello";

fn main() {
    println!("Starts to upload simple data for an example");

    let auth = Auth {
        access_key: ACCESS_KEY.to_string(),
        secret_key: SECRET_KEY.to_string(),
    };

    let uploader = DataUploader::new(auth);
    let data = Arc::new(DATA.as_bytes().to_vec());

    uploader.upload(BUCKET, KEY, EXPIRED_SECONDS, data).unwrap();

    println!("Finishes data upload");
}
