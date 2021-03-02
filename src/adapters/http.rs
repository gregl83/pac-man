use std::io::{
    Error,
    ErrorKind
};
use futures::stream::{StreamExt, TryStreamExt};
use http::Request;
use hyper::{
    Client,
    header::HeaderValue,
    HeaderMap,
    Body
};
use hyper_tls::HttpsConnector;
use bytes::{BytesMut, BufMut};

use crate::adapters::BodyStream;

type Headers = Vec<(String, String)>;

pub async fn get_stream(headers: &Headers, uri: &str) -> (HeaderMap, BodyStream) {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let mut builder = Request::get(uri);
    for (header, value) in headers {
        builder = builder.header(header, value);
    }
    let request = builder.body(Body::empty()).unwrap();

    let response = client.request(request).await.unwrap();
    let mut headers = response.headers().clone();
    let mut body = response.into_body();

    if !headers.contains_key("content-length") {
        let mut bytes = BytesMut::new();
        while let Some(next) = body.next().await {
            bytes.put(next.unwrap());
        }
        headers.insert("content-length", HeaderValue::from(bytes.len()));
        body = Body::from(bytes.freeze());
    }

    (headers, Box::new(body.map_err(|e| Error::new(ErrorKind::Other, e))))
}