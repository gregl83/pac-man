use std::str::FromStr;

use rusoto_core::Region;
use rusoto_s3::{
    S3,
    S3Client,
    PutObjectRequest,
    PutObjectOutput,
    HeadObjectRequest,
    HeadObjectOutput,
    StreamingBody
};

use crate::adapters::BodyStream;

pub async fn put_object<'a>(
    region: &'a str,
    bucket: &'a str,
    filename: &'a str,
    content_type: &'a str,
    content_length: i64,
    body: BodyStream
) -> PutObjectOutput {
    let region = Region::from_str(region).unwrap();
    let stream = StreamingBody::new(body);
    let client = S3Client::new(region);
    client.put_object(PutObjectRequest {
        bucket: String::from(bucket),
        key: String::from(filename),
        content_type: Some(String::from(content_type)),
        content_length: Some(content_length),
        body: Some(stream),
        ..Default::default()
    }).await.unwrap()
}

pub async fn get_object_head<'a>(
    region: &'a str,
    bucket: &'a str,
    filename: &'a str,
) -> HeadObjectOutput {
    let region = Region::from_str(region).unwrap();
    let client = S3Client::new(region);
    client.head_object(HeadObjectRequest {
        bucket: String::from(bucket),
        key: String::from(filename),
        ..Default::default()
    }).await.unwrap()
}