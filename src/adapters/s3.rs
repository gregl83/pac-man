use rusoto_core::Region;
use rusoto_s3::{
    S3,
    S3Client,
    PutObjectRequest,
    PutObjectOutput,
    StreamingBody
};

use crate::adapters::BodyStream;

pub async fn put_object(content_length: i64, body: BodyStream) -> PutObjectOutput {
    let stream = StreamingBody::new(body);
    let client = S3Client::new(Region::UsEast1); // fixme - dynamic region
    client.put_object(PutObjectRequest {
        bucket: String::from("rust-pac-man"),
        key: String::from("filename"),
        content_length: Some(content_length),
        body: Some(stream),
        ..Default::default()
    }).await.unwrap()
}