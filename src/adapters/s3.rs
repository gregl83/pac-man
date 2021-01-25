use std::io::Result;
use futures::Stream;
use bytes::Bytes;
use rusoto_core::Region;
use rusoto_s3::{
    S3,
    S3Client,
    PutObjectRequest,
    PutObjectOutput,
    StreamingBody
};

type BodyStream = Box<dyn Stream<Item = Result<Bytes>> + Send + Sync + Unpin>;

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