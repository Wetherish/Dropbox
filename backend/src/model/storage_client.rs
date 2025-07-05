use s3::{Bucket, BucketConfiguration, Region, creds::Credentials};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StorageFileName {
    pub file_id: surrealdb::RecordId,
    pub name: String,
}

#[derive(Clone)]
pub struct BucketClient {
    pub bucket: Box<Bucket>,
}

impl BucketClient {
    pub async fn new() -> BucketClient {
        let bucket_name = "test-rust-s3";
        let region = Region::Custom {
            region: "".to_owned(),
            endpoint: "http://localhost:9000".to_owned(),
        };
        let credentials = Credentials {
            access_key: Some("minioadmin".to_owned()),
            secret_key: Some("minioadmin".to_owned()),
            security_token: None,
            session_token: None,
            expiration: None,
        };
        let mut bucket = Bucket::new(bucket_name, region.clone(), credentials.clone())
            .unwrap()
            .with_path_style();

        if !bucket.exists().await.unwrap() {
            bucket = Bucket::create_with_path_style(
                bucket_name,
                region,
                credentials,
                BucketConfiguration::default(),
            )
            .await
            .unwrap()
            .bucket;
        }
        BucketClient { bucket }
    }
}
