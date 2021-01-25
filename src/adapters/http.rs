use std::io::{
    Error,
    ErrorKind,
    Result
};
use futures::stream::{
    Stream,
    TryStreamExt
};
use bytes::Bytes;
use http::Request;
use hyper::{
    Client,
    HeaderMap,
    Body,
};
use hyper_tls::HttpsConnector;

type BodyStream = Box<dyn Stream<Item = Result<Bytes>> + Send + Sync + Unpin>;

pub async fn get_stream() -> (HeaderMap, BodyStream) {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let request = Request::get("http://httpbin.org/ip").body(Body::empty()).unwrap(); // fixme - dynamic request
    let response = client.request(request).await.unwrap();

    let headers = response.headers().clone();
    let body = response
        .into_body()
        .map_err(|e| Error::new(ErrorKind::Other, e));

    (headers, Box::new(body))
}