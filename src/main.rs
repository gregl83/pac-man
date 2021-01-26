mod adapters;

use lambda::{handler_fn, Context};
use log::{LevelFilter, error};
use simple_logger::SimpleLogger;
use simple_error::bail;
use serde_json::Value;
use futures::stream::StreamExt;

use adapters::{
    http,
    s3,
    to_uri
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    lambda::run(handler_fn(func)).await?;

    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
    // todo - move source parsing w/err handles to fn
    let scheme = event["source"]["scheme"].as_str().unwrap();
    let credentials = Some(
        (
            event["source"]["username"].as_str().unwrap(),
            event["source"]["password"].as_str().unwrap(),
        )
    );
    let hostname = event["source"]["hostname"].as_str().unwrap();
    let port = event["source"]["port"].as_u64();
    let path = event["source"]["path"].as_str();
    let params = event["source"]["params"].as_object();
    let fragment = event["source"]["fragment"].as_str();
    let uri = to_uri(scheme, credentials, hostname, port, path, params, fragment);

    let (headers, body) = http::get_stream(&uri).await;
    let content_length: i64 = headers
        .get("content-length")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();

    let region = event["destination"]["region"].as_str().unwrap();
    let collection = event["destination"]["collection"].as_str().unwrap();
    let name = event["destination"]["name"].as_str().unwrap();

    s3::put_object(
        region,
        collection,
        name,
        content_length,
        body
    ).await;

    Ok(event)
}