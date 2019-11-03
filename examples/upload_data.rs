use qiniu_rs::upload::data_uploader::DataUploader;

fn main() {
    println!("Starts to upload simple data for an example");

    let buf = vec!['h', 'e', 'l', 'l', 'o'];

    let uploader = DataUploader::new();
    uploader.upload(buf);

    println!("Finishes data upload");
}
