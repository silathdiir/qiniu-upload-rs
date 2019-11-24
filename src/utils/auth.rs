use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

struct Auth {
    access_key: String,
    secret_key: String,
}

impl Auth {
    pub fn upload_token(
        &self,
        bucket: &str,
        key: &str,
        expired_seconds: u64,
    ) -> String {
        let json_data = json!({
            "scope": self.scope(bucket, key),
            "deadline": self.deadline(expired_seconds),
        }).to_string();

        let data = base64::encode_config(&json_data, base64::URL_SAFE);

        let sha1_digest = self.sha1_digest(&data);
        let digest = base64::encode_config(&sha1_digest, base64::URL_SAFE);

        format!("{}:{}:{}", self.access_key, digest, data)
    }

    fn deadline(&self, expired_seconds: u64) -> u64 {
        if expired_seconds == 0 {
            panic!("param `expired_seconds` must be positive")
        }

        let time_now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        time_now_secs + expired_seconds
    }

    fn scope(&self, bucket: &str, key: &str) -> String {
        if bucket.is_empty() {
            panic!("param `bucket` cannot be empty")
        }

        if key.is_empty() {
            panic!("param `key` cannot be empty")
        }

        format!("{}:{}", bucket, key)
    }

    fn sha1_digest(&self, data: &str) -> Vec<u8> {
        let mut hmac = Hmac::new(Sha1::new(), self.secret_key.as_bytes());
        hmac.input(data.as_bytes());

        let mut vec: Vec<u8> = Vec::new();
        vec.copy_from_slice(hmac.result().code());
        vec
    }
}
