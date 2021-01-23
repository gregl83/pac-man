mod adapters;

use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{LevelFilter, error};
use simple_logger::SimpleLogger;
use simple_error::bail;
use serde::{Deserialize, Serialize};

use adapters::http;

#[derive(Serialize, Deserialize)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    content: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    lambda!(my_handler);

    Ok(())
}

async fn my_handler(e: CustomEvent, c: Context) -> Result<Data, HandlerError> {
    if e.first_name == "" {
        error!("Empty first name in request {}", c.aws_request_id);
        bail!("Empty first name");
    }

    let mut res = String::new();
    let stream = http::get_stream();

    while let Some(chunk) = stream.await {
        res.push_str(&chunk?);
        //stdout().write_all(&chunk?).await?;
    }

    Ok(Data {
        content: format!("{}", res),
    })
}