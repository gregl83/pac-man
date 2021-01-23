use hyper::{
    body::HttpBody,
    Client,
};
use tokio::io::{
    self,
    AsyncWriteExt
};

type Data = Box<dyn HttpBody + Unpin + ?Sized>;
type HttpResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn get_stream() -> HttpResult<Data> {
    let client = Client::new();

    let uri = "http://httpbin.org/ip".parse()?;

    let mut resp = client.get(uri).await?;

    Ok(Box::new(resp.body_mut().data()))
}

// todo - build requests wrapper!!!