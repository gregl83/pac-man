use std::fmt;
use std::str::FromStr;
use std::ops::{Range, Add, Sub};
use std::collections::HashMap;

use serde_json::{
    map::Map,
    Value
};
use futures::executor::block_on;
use regex::Regex;
use rusoto_core::Region;
use rusoto_secretsmanager::{SecretsManagerClient, SecretsManager, GetSecretValueRequest};

type Name = String;
type Key = String;
type Secret = String;

/// Secrets connects to AWS Secrets Manager
pub struct Secrets {
    client: SecretsManagerClient,
    cache: HashMap<String, Option<Secret>>
}

impl Secrets {
    pub fn new(region: &str) -> Self {
        let region = Region::from_str(region).unwrap();
        Secrets {
            client: SecretsManagerClient::new(region),
            cache: HashMap::new()
        }
    }

    /// Fill secrets into given string pattern
    ///
    /// Replaces:  {:secrets:<key>}
    /// With:      <value-for-key>
    pub async fn fill(&mut self, pattern: &String) -> String {
        let mut res = String::new();

        let re = Regex::new(r"\{:secrets:([^:]+):([^}]+)}").unwrap();
        let mut capture_matches = re.captures_iter(&pattern);

        let mut capture_name = "";
        let mut capture_key = "";
        let mut capture_start = 0;
        let mut capture_end = 0;
        for (i, c) in pattern.chars().enumerate() {
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

    /// Get secret associated with given key
    async fn get(&mut self, n: Name, k: Key) -> Option<Secret> {
        Some(String::from(""))

        //let cache_key = format!("{}:{}", &n, &k);

        // if let Some(s) = self.cache.get_mut(cache_key.as_str()) { return s.clone(); }
        //
        // if let Some(s) = self.fetch(&n).await {
        //     let secret: Value = serde_json::from_str(s.as_str()).unwrap();
        //     for (key, value) in secret.as_object().unwrap().iter() {
        //         let cache_key = format!("{}:{}", &n, &key);
        //         let value = Some(String::from(value.as_str().unwrap()));
        //         self.cache.insert(cache_key, value);
        //     }
        //     return self.cache.get(format!("{}:{}", n, k).as_str()).unwrap().clone()
        // }
        //
        // self.cache.insert(cache_key, None);
        //None
    }

    /// Fetch secret from AWS Secrets Manager
    async fn fetch(&mut self, n: &Name) -> Option<Secret> {
        let s = self.client.get_secret_value(GetSecretValueRequest {
            secret_id: n.to_string(),
            version_id: None,
            version_stage: None,
        }).await.unwrap();
        s.secret_string
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