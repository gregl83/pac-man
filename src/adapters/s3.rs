use rusoto_core::{Region, ByteStream};
use rusoto_s3::{
    S3,
    S3Client,
    PutObjectRequest,
    PutObjectOutput,
    StreamingBody
};
use hyper::Body;


pub async fn put_object(body: Body) -> PutObjectOutput {
    let stream = ByteStream::new(body);
    let client = S3Client::new(Region::UsEast1); // fixme - dynamic region
    client.put_object(PutObjectRequest {
        bucket: String::from("bucket"),
        key: String::from("key"),
        body: Some(stream),
        ..Default::default()
    }).await.unwrap()
}