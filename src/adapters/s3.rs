use std::str::FromStr;

use rusoto_core::Region;
use rusoto_s3::{
    S3,
    S3Client,
    PutObjectRequest,
    PutObjectOutput,
    StreamingBody
};

use crate::adapters::BodyStream;

pub async fn put_object(
    region: &str,
    bucket: &str,
    filename: &str,
    content_length: i64,
    body: BodyStream
) -> PutObjectOutput {
    let region = Region::from_str(region).unwrap();
    let stream = StreamingBody::new(body);
    let client = S3Client::new(region);
    client.put_object(PutObjectRequest {
        bucket: String::from(bucket),
        key: String::from(filename),
        content_length: Some(content_length),
        body: Some(stream),
        ..Default::default()
    }).await.unwrap()
}