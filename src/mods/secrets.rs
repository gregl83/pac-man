use std::collections::HashMap;

use regex::Regex;
use serde_json::Value;

use crate::adapters::secrets::get_secret;
use crate::mods::Modifier;

pub const NAME: &str = "secrets";

/// Secrets connects to AWS Secrets Manager
pub struct Secrets {
    region: String,
    cache: HashMap<String, Option<String>>
}

impl Secrets {
    pub fn new(region: &str) -> Self {
        Secrets {
            region: String::from(region),
            cache: HashMap::new()
        }
    }

    /// Get secret by name and key
    ///
    /// Uses cache or async service call
    async fn get(&mut self, n: &str, k: &str) -> Option<String> {
        let cache_key = format!("{}:{}", n, k);

        if let Some(s) = self.cache.get_mut(cache_key.as_str()) { return s.clone(); }

        if let Some(s) = self.fetch(String::from(n)).await {
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
        get_secret(self.region.clone(), n, None, None).await
    }
}

#[async_trait::async_trait]
impl Modifier for Secrets {
    fn key(&self) -> &'static str { NAME }

    /// Modify secrets patterns in target string
    ///
    /// Replaces:  {:secrets:<key>}
    /// With:      <value-for-key>
    async fn modify(&mut self, params: Vec<&str>) -> Option<String> {
        self.get(params[0], params[1]).await
    }
}

// todo - add tests back (removed due to trait async support w/mocks) --
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn secrets_get_uncached() {
//         let key = String::from("key");
//         let expects = None;
//
//         let mut secrets_manager = Box::new(MockSecretsManagerClient::new());
//         let mut secrets = Secrets::new(secrets_manager);
//         let actual = secrets.get(&key);
//
//         assert_eq!(actual, expects);
//     }
//
//     #[test]
//     fn secrets_miss_returns_none() {
//         let key = String::from("key");
//         let expects = None;
//
//         let mut secrets_manager = Box::new(MockSecretsManagerClient::new());
//         let mut secrets = Secrets::new(secrets_manager);
//         let actual = secrets.get(&key);
//
//         assert_eq!(actual, expects);
//     }
//
//     #[test]
//     async fn secrets_miss_cached() {
//         let mut secrets_manager = Box::new(MockSecretsManagerClient::new());
//         let mut secrets = Secrets::new(secrets_manager);
//         // todo - assert fetch is NOT called
//     }
// }