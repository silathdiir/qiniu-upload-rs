use qiniu_rs::upload::data_uploader::DataUploader;
use qiniu_rs::utils::auth::Auth;

const ACCESS_KEY: &str = "";
const SECRET_KEY: &str = "";

const BUCKET: &str = "";
const KEY: &str = "";
const DATA: &str = "hello";

const EXPIRED_SECONDS: u64 = 3600;

fn main() {
    println!("Starts to upload simple data for an example");

    let auth = Auth { access_key: ACCESS_KEY.to_string(), secret_key: SECRET_KEY.to_string(), };
    let uploader = DataUploader::new(auth);
    uploader.upload(BUCKET, KEY, EXPIRED_SECONDS, &DATA.as_bytes()).unwrap();

    println!("Finishes data upload");
}
