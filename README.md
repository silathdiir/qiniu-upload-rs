# qiniu-upload-rs

This project focusses on implementing the `Upload` functions for
[Qiniu](https://www.qiniu.com/) Cloud Service. It tries to pack the sufficient
and efficient Upload Client functions only for Qiniu Upload, and except other
Qiniu services.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
qiniu-upload-rs = "0.0.1"
```

You could checkout the user code in folder `examples/upload_data.rs`, and update
constants `ACCESS_KEY`, `SECRET_KEY` and `BUCKET` of your own Qiniu account.
Then run this example as below.

```
cargo run --example upload_data
```

After running the above command, you could check if the test file is uploaded
successfully on Qiniu Admin Web.
