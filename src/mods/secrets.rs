use std::collections::HashMap;
use std::future::Future;

use serde_json::Value;
use futures::executor::block_on;

use crate::mods::Modifier;

pub const NAME: &str = "secrets";

/// Secrets connects to AWS Secrets Manager
pub struct Secrets<F, Fut>
where
    F: Fn(String, String, Option<String>, Option<String>) -> Fut + Send,
    Fut: Future<Output = Option<String>>
{
    region: String,
    cache: HashMap<String, Option<String>>,
    fetcher: F
}

impl<F, Fut> Secrets<F, Fut>
where
    F: Fn(String, String, Option<String>, Option<String>) -> Fut + Send,
    Fut: Future<Output = Option<String>>
{
    pub fn new(region: &str, fetcher: F) -> Self {
        Secrets {
            region: String::from(region),
            cache: HashMap::new(),
            fetcher
        }
    }

    /// Get secret by name and key
    ///
    /// Uses cache or async service call
    async fn get(&mut self, n: &str, k: &str) -> Option<String> {
        let cache_key = format!("{}:{}", n, k);

        if let Some(s) = self.cache.get_mut(cache_key.as_str()) { return s.clone(); }

        if let Some(s) = block_on(self.fetch(String::from(n))) {
            let secret: Value = serde_json::from_str(s.as_str()).unwrap();
            for (key, value) in secret.as_object().unwrap().iter() {
                let cache_key = format!("{}:{}", &n, &key);
                let value = Some(String::from(value.as_str().unwrap()));
                self.cache.insert(cache_key, value);
            }
            return self.cache.get(format!("{}:{}", n, k).as_str()).unwrap().clone()
        }

        self.cache.insert(cache_key, None);
        None
    }

    /// Fetch secret using secrets adapter
    async fn fetch(&self, n: String) -> Option<String> {
        (self.fetcher)(self.region.clone(), n, None, None).await
    }
}

#[async_trait::async_trait]
impl<F, Fut> Modifier for Secrets<F, Fut>
where
    F: Fn(String, String, Option<String>, Option<String>) -> Fut + Send,
    Fut: Future<Output = Option<String>>
{
    fn key(&self) -> &'static str { NAME }

    /// Modify secrets patterns in target string
    ///
    /// Replaces:  {:secrets:<key>}
    /// With:      <value-for-key>
    async fn modify(&mut self, params: Vec<&str>) -> Option<String> {
        self.get(params[0], params[1]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn secrets_get_uncached() {
        let n = String::from("namespace");
        let k = String::from("key");
        let expects = Some(String::from("value"));

        async fn fetcher(_: String, _: String, _: Option<String>, _: Option<String>) -> Option<String> {
            Some(String::from("{\"key\":\"value\"}"))
        };

        let mut secrets = Secrets::new("us-east-1", fetcher);

        let actual = secrets.get(&n, &k).await;

        assert_eq!(actual, expects);
    }

    #[tokio::test]
    async fn secrets_miss_returns_none() {
        let n = String::from("namespace");
        let k = String::from("key");
        let expects = None;

        async fn fetcher(_: String, _: String, _: Option<String>, _: Option<String>) -> Option<String> {
            None
        };

        let mut secrets = Secrets::new("us-east-1", fetcher);

        let actual = secrets.get(&n, &k).await;

        assert_eq!(actual, expects);
    }
}