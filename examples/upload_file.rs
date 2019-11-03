use qiniu_rs::upload::file_uploader::FileUploader;

fn main() {
    println!("Starts to upload a file for an example");

    let file_path = "examples/data/simple.txt";

    let uploader = DataUploader::new();
    uploader.upload(file_path);

    println!("Finishes file upload");
}