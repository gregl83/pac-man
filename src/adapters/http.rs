use std::io::{
    Error,
    ErrorKind
};
use futures::stream::TryStreamExt;
use http::Request;
use hyper::{
    Client,
    HeaderMap,
    Body,
};
use hyper_tls::HttpsConnector;

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

    let headers = response.headers().clone();
    let body = response
        .into_body()
        .map_err(|e| Error::new(ErrorKind::Other, e));

    (headers, Box::new(body))
}