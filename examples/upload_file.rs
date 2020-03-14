use qiniu_upload_rs::{upload::file_uploader::FileUploader, utils::auth::Auth};

const ACCESS_KEY: &str = "test-access-key";
const SECRET_KEY: &str = "test-secret-key";

const BUCKET: &str = "test-bucket";
const KEY: &str = "test-file-key.txt";
const EXPIRED_SECONDS: u64 = 3600;

const FILE_PATH: &str = "examples/fixtures/simple.txt";

fn main() {
    println!("Starts to upload a file for an example");

    let auth = Auth {
        access_key: ACCESS_KEY.to_string(),
        secret_key: SECRET_KEY.to_string(),
    };

    let uploader = FileUploader::new(auth);

    uploader
        .upload(BUCKET, KEY, EXPIRED_SECONDS, FILE_PATH)
        .unwrap();

    println!("Finishes file upload");
}
