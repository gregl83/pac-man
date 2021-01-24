mod adapters;

use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{LevelFilter, error};
use simple_logger::SimpleLogger;
use simple_error::bail;
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;

use adapters::http;

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

    let (headers, mut body) = http::get_stream().await;

    while let Some(chunk) = body.next().await {
        println!("{:?}", chunk);
    }

    Ok(())
    //
    // println!("{:?}", res);

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