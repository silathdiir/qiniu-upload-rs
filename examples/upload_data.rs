use qiniu_rs::upload::data_uploader::DataUploader;

fn main() {
    println!("Starts to upload simple data for an example");

    let buf = "hello";

    let uploader = DataUploader::new();
    uploader.upload("dummy_upload_data", &buf.as_bytes().to_vec());

    println!("Finishes data upload");
}
