pub mod http;
pub mod s3;

use std::io;

use bytes::Bytes;
use futures::Stream;
use serde_json::{
    map::Map,
    Value
};

type QueryParams = Map<String, Value>;
type BodyStream = Box<dyn Stream<Item = io::Result<Bytes>> + Send + Sync + Unpin>;

/// Construct Query from Params Map
fn to_query(params: QueryParams) -> String {
    let mut num_params = params.len();
    let mut query = String::new();

    if num_params == 0 { return query; }

    query.push_str("?");
    for (name, value) in params.iter() {
        let part = format!("{}={}", name, value.as_str().unwrap());
        query.push_str(part.as_str());
        num_params -= 1;
        if num_params > 0 { query.push_str("&"); }
    }
    query
}

/// Construct URI string from parts
///
/// Assumption:
///
/// <scheme>://<username>:<password>@<subdomain.domain.tld>:<port><path>?<query>#<fragment>
///
/// Used to process URIs from configuration based functions
pub fn to_uri(
    scheme: &str,
    username: Option<&str>,
    password: Option<&str>,
    hostname: &str,
    port: Option<u16>,
    path: &str,
    params: QueryParams
) -> Result<String, io::Error> {
    Ok(format!(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_query_empty() {
        let params = Map::new();
        let expect = String::new();
        let actual = to_query(params);
        assert_eq!(actual, expect);
    }

    #[test]
    fn to_query_single_param() {
        let mut params = Map::new();
        params.insert(String::from("name"), Value::from("value"));
        let expect = String::from("?name=value");
        let actual = to_query(params);
        assert_eq!(actual, expect);
    }

    #[test]
    fn to_query_with_params() {
        let mut params = Map::new();
        params.insert(String::from("a"), Value::from("alpha"));
        params.insert(String::from("b"), Value::from("bravo"));
        params.insert(String::from("c"), Value::from("charlie"));
        let expect = String::from("?a=alpha&b=bravo&c=charlie");
        let actual = to_query(params);
        assert_eq!(actual, expect);
    }
}