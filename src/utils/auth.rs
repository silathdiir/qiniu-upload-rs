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

        hmac.result().code().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DUMMY_ACCESS_KEY: &str = "dummy_access_key";
    const DUMMY_SECRET_KEY: &str = "dummy_secret_key";
    const DUMMY_BUCKET: &str = "dummy_bucket";
    const DUMMY_KEY: &str = "dummy_key";

    #[test]
    fn generate_upload_token() {
        let auth = Auth {
            access_key: DUMMY_ACCESS_KEY.to_string(),
            secret_key: DUMMY_SECRET_KEY.to_string(),
        };

        let new_upload_token = auth.upload_token(
            DUMMY_BUCKET,
            DUMMY_KEY,
            600,
        );

        let items: Vec<&str> = new_upload_token.split(":").collect();
        assert_eq!(items[0], "dummy_access_key");
    }
}
