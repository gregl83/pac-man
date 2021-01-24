use hyper::{
    Client,
    HeaderMap,
    Body,
};
use hyper_tls::HttpsConnector;
use http::Request;

pub async fn get_stream() -> (HeaderMap, Body) {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let request = Request::get("http://httpbin.org/ip").body(Body::empty()).unwrap(); // fixme - dynamic request
    let response = client.request(request).await.unwrap();

    (response.headers().clone(), response.into_body())
}