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
    secrets::Secrets,
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
    let region = event["secrets"]["region"].as_str().unwrap();
    let mut secrets = Secrets::new(region);

    let uri = source_to_uri(&event["source"]);
    let uri = secrets.fill(&uri).await;
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

/// Event Source to URI  - Checks for optional parts
fn source_to_uri(source: &Value) -> String {
    let scheme = source["scheme"].as_str().unwrap();

    let username = source.get("username");
    let password = source.get("password");
    let credentials = match (username, password) {
        (Some(u), Some(p)) => {
            Some((u.as_str().unwrap(), p.as_str().unwrap()))
        }
        _ => None
    };

    let hostname = source["hostname"].as_str().unwrap();

    let port = match source.get("port") {
        Some(port) => port.as_u64(),
        _ => None
    };
    let path = match source.get("path") {
        Some(v) => v.as_str(),
        _ => None
    };
    let params = match source.get("params") {
        Some(v) => v.as_object(),
        _ => None
    };
    let fragment = match source.get("fragment") {
        Some(v) => v.as_str(),
        _ => None
    };

    to_uri(scheme, credentials, hostname, port, path, params, fragment)
}