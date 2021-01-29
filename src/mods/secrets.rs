use std::collections::HashMap;

use regex::Regex;
use futures::executor::block_on;
use serde_json::Value;

use crate::adapters::secrets::get_secret;
use crate::mods::Modifier;

type Name = String;
type Key = String;
type Secret = String;

/// Secrets connects to AWS Secrets Manager
pub struct Secrets {
    region: String,
    cache: HashMap<String, Option<Secret>>
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
    async fn get(&mut self, n: Name, k: Key) -> Option<Secret> {
        let cache_key = format!("{}:{}", &n, &k);

        if let Some(s) = self.cache.get_mut(cache_key.as_str()) { return s.clone(); }

        if let Some(s) = self.fetch(n.clone()).await {
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
    async fn fetch(&self, n: Name) -> Option<String> {
        get_secret(self.region.clone(), n, None, None).await
    }
}

#[async_trait::async_trait]
impl Modifier<String> for Secrets {
    /// Modify secrets patterns in target string
    ///
    /// Replaces:  {:secrets:<key>}
    /// With:      <value-for-key>
    async fn modify(&mut self, target: String) -> String {
        let mut res = String::new();

        let re = Regex::new(r"\{:secrets:([^:]+):([^}]+)}").unwrap();
        let mut capture_matches = re.captures_iter(&target);

        let mut capture_name = "";
        let mut capture_key = "";
        let mut capture_start = 0;
        let mut capture_end = 0;
        for (i, c) in target.chars().enumerate() {
            if i == capture_end {
                if let Some(next_captures) = capture_matches.next() {
                    capture_name = next_captures.get(1).unwrap().as_str();
                    capture_key = next_captures.get(2).unwrap().as_str();
                    let captures_match = next_captures.get(0).unwrap().range();
                    capture_start = captures_match.start;
                    capture_end = captures_match.end;
                }
            }

            if i == capture_start && !capture_name.is_empty() && !capture_key.is_empty() {
                let name = capture_name.to_string();
                let key = capture_key.to_string();
                if let Some(secret) = block_on(self.get(name, key)) {
                    res.push_str(secret.as_str());
                }
                continue;
            }

            if i < capture_start || i >= capture_end { res.push(c); }
        }

        res
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