use qiniu_upload_rs::upload::file_uploader::FileUploader;

fn main() {
    println!("Starts to upload a file for an example");

    let file_path = "examples/data/simple.txt";

    let uploader = FileUploader::new();
    uploader.upload("dummy_upload_file", file_path);

    println!("Finishes file upload");
}
