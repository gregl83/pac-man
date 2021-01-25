mod adapters;

use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{LevelFilter, error};
use simple_logger::SimpleLogger;
use simple_error::bail;
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;

use adapters::{
    http,
    s3
};

#[derive(Serialize, Deserialize)]
struct CustomEvent {
    id: i8,
}

#[derive(Serialize, Deserialize)]
struct Data {
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let uri = "https://demo.ckan.org/api/action/package_search?facet.field=[%22tags%22]&facet.limit=1000000&rows=0";
    let (headers, body) = http::get_stream(uri).await;
    let content_length: i64 = headers
        .get("content-length")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();

    let result = s3::put_object(content_length, body).await;

    println!("{:?}", result);

    Ok(())

    //lambda!(my_handler);
}

async fn my_handler(e: CustomEvent, c: Context) -> Result<Data, HandlerError> {
    // if e.id == "" {
    //     error!("Empty id in request {}", c.aws_request_id);
    //     bail!("Empty id");
    // }

    Ok(Data {
        content: format!("id: {}", e.id),
    })
}