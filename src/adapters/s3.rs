use rusoto_core::Region;
use rusoto_s3::{
    S3,
    S3Client,
    PutObjectRequest,
    PutObjectOutput,
    StreamingBody
};

use crate::adapters::BodyStream;

pub async fn put_object(body: BodyStream) -> PutObjectOutput {
    let stream = StreamingBody::new(body);
    let client = S3Client::new(Region::UsEast1); // fixme - dynamic region
    client.put_object(PutObjectRequest {
        bucket: String::from("bucket"),
        key: String::from("key"),
        body: Some(stream),
        ..Default::default()
    }).await.unwrap()
}