use std::sync::Arc;
use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;

#[derive(Clone)]
pub struct S3Client {
    host: String,
    access_key_id: String,
    secret_access_key: String,
    region: String,
    bucket: String
}

impl S3Client {
    pub fn new(host: &str, access_key_id: &str, secret_access_key: &str, region: &str, bucket: &str) -> S3Client {
        Self {
            host: host.to_owned(),
            access_key_id: access_key_id.to_owned(),
            secret_access_key: secret_access_key.to_owned(),
            region: region.to_owned(),
            bucket: bucket.to_owned(),
        }
    }

    async fn get_client(&self) -> Arc<Client> {
        let config = aws_config::from_env()
            .endpoint_url(&self.host)
            .region(Region::new(self.region.clone()))
            .credentials_provider(Credentials::new(
                &self.access_key_id,
                &self.secret_access_key,
                None,
                None,
                "static",
            ))
            .load()
            .await;

        Arc::new(Client::new(&config))
    }

    pub async fn upload_image(
        &self,
        extension_name: &str,
        content_type: &str,
        bytes: bytes::Bytes
    ) -> Result<String, aws_sdk_s3::Error> {
        let body = ByteStream::from(bytes);
        let generate_file_name = uuid::Uuid::new_v4().to_string();
        let client = self.get_client().await;
        let key_name = String::from(generate_file_name.clone() + "." + &extension_name);

        client
            .put_object()
            .bucket(&self.bucket)
            .key(&key_name)
            .body(body)
            .content_type(content_type)
            .send()
            .await?;

        Ok(key_name)
    }
}