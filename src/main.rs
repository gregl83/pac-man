mod adapters;

use lambda::{handler_fn, Context};
use log::{LevelFilter, error};
use simple_logger::SimpleLogger;
use simple_error::bail;
use serde_json::Value;
use futures::stream::StreamExt;

use adapters::{
    http,
    s3
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let func = handler_fn(func);

    lambda::run(func).await?;

    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
    let uri = "https://demo.ckan.org/api/action/package_search?facet.field=[%22tags%22]&facet.limit=1000000&rows=0";
    let (headers, body) = http::get_stream(uri).await;
    let content_length: i64 = headers
        .get("content-length")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();

    let _ = s3::put_object(
        "us-east-1",
        "rust-pac-man",
        "filename",
        content_length,
        body
    ).await;

    Ok(event)
}