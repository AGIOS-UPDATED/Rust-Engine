use aws_sdk_s3::{Client, Error};
use aws_sdk_s3::model::PutObjectRequest;
use aws_config::meta::region::RegionProviderChain;
use aws_config::ConfigLoader;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct S3Config {
    pub bucket_name: String,
    pub region: String,
}

impl S3Config {
    pub fn new() -> Result<Self, envy::Error> {
        dotenv::dotenv().ok();
        envy::from_env()
    }
}

pub async fn upload_to_s3(file_path: &str, file_name: &str) -> Result<String, Error> {
    // Load AWS configuration
    let config = aws_config::load_from_env().await;
    let s3_client = Client::new(&config);

    // Read the file into memory
    let file_data = tokio::fs::read(file_path).await?;

    // Create PutObjectRequest
    let put_request = PutObjectRequest::builder()
        .bucket(file_name)
        .key(file_name)
        .body(file_data.into())
        .build();

    // Upload file to S3
    let response = s3_client.put_object(put_request).send().await?;

    Ok(format!("File uploaded to S3 with ETag: {}", response.e_tag().unwrap_or_default()))
}
