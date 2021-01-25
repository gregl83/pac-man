use std::io::{
    Error,
    ErrorKind
};
use futures::stream::{
    Stream,
    TryStreamExt
};
use http::Request;
use hyper::{
    Client,
    HeaderMap,
    Body,
};
use hyper_tls::HttpsConnector;

use crate::adapters::BodyStream;

pub async fn get_stream(uri: &str) -> (HeaderMap, BodyStream) {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let request = Request::get(uri).body(Body::empty()).unwrap();
    let response = client.request(request).await.unwrap();

    let headers = response.headers().clone();
    let body = response
        .into_body()
        .map_err(|e| Error::new(ErrorKind::Other, e));

    (headers, Box::new(body))
}