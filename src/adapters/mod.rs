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
fn to_query(params: &QueryParams) -> String {
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

/// Construct URI String from parts
///
/// Assumption:
/// <scheme>://<username>:<password>@<subdomain.domain.tld>:<port><path>?<query>#<fragment>
///
/// Used to process URIs from configuration based functions
///
/// todo - all kinds of splendid err handlin
pub fn to_uri<'a>(
    scheme: &'a str,
    credentials: Option<(&'a str, &'a str)>,
    hostname: &'a str,
    port: Option<u64>,
    path: Option<&'a str>,
    params: Option<&'a QueryParams>,
    fragment: Option<&'a str>
) -> String {
    let mut uri = format!("{}://", scheme);

    if let Some((username, password)) = credentials {
        uri.push_str(format!("{}:{}@", username, password).as_str());
    }

    uri.push_str(hostname);

    if let Some(port) = port {
        uri.push_str(format!(":{}", port).as_str());
    }

    uri.push('/');
    if let Some(path) = path {
        if path.chars().next().unwrap() == '/' {
            uri.push_str(&path[1..]);
        } else {
            uri.push_str(path);
        }
    }

    if let Some(params) = params {
        uri.push_str(to_query(params).as_str());
    }

    if let Some(fragment) = fragment {
        uri.push_str(format!("#{}", fragment).as_str());
    }

    uri
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_query_empty() {
        let params = Map::new();
        let expect = String::new();
        let actual = to_query(&params);
        assert_eq!(actual, expect);
    }

    #[test]
    fn to_query_single_param() {
        let mut params = Map::new();
        params.insert(String::from("name"), Value::from("value"));
        let expect = String::from("?name=value");
        let actual = to_query(&params);
        assert_eq!(actual, expect);
    }

    #[test]
    fn to_query_with_params() {
        let mut params = Map::new();
        params.insert(String::from("a"), Value::from("alpha"));
        params.insert(String::from("b"), Value::from("bravo"));
        params.insert(String::from("c"), Value::from("charlie"));
        let expect = String::from("?a=alpha&b=bravo&c=charlie");
        let actual = to_query(&params);
        assert_eq!(actual, expect);
    }

    #[test]
    fn to_uri_basic() {
        let scheme = "https";
        let credentials = None;
        let hostname = "host.name.com";
        let port = None;
        let path = None;
        let params = None;
        let fragment = None;

        let expect = String::from("https://host.name.com/");
        let actual = to_uri(scheme, credentials, hostname, port, path, params, fragment);
        assert_eq!(actual, expect);
    }

    #[test]
    fn to_uri_complex() {
        let scheme = "https";
        let credentials = Some(("username", "password"));
        let hostname = "host.name.com";
        let port: Option<u64> = Some(8080);
        let path = Some("/yellow/brick/road");
        let mut params = Map::new();
        params.insert(String::from("name"), Value::from("value"));
        let params = Some(&params);
        let fragment = Some("/follow/the");

        let expect = String::from("https://username:password@host.name.com:8080/yellow/brick/road?name=value#/follow/the");
        let actual = to_uri(scheme, credentials, hostname, port, path, params, fragment);
        assert_eq!(actual, expect);
    }
}