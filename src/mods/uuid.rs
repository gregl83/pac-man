use std::collections::HashMap;

use uuid;

use crate::mods::Modifier;

pub const NAME: &str = "uuid";

/// Uuid generates uuid v4
pub struct Uuid {
    cache: HashMap<String, String>
}

impl Uuid {
    pub fn new() -> Self {
        Uuid {
            cache: HashMap::new()
        }
    }
}

#[async_trait::async_trait]
impl Modifier for Uuid {
    fn key(&self) -> &'static str { NAME }

    async fn modify(&mut self, params: Vec<&str>) -> Option<String> {
        if let Some(key) = params.get(0) {
            let uuid = self.cache
                .entry(String::from(*key))
                .or_insert(uuid::Uuid::new_v4().to_string())
                .clone();
            return Some(uuid);
        }
        Some(uuid::Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use regex::Regex;

    const UUID_V4_PATTERN: &str = "^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$";

    #[tokio::test]
    async fn uuid_modify_sans_key() {
        let params = vec![];

        let mut uuid = Uuid::new();
        let actual = uuid.modify(params).await;

        let uuid_v4 = Regex::new(UUID_V4_PATTERN).unwrap();

        assert!(uuid_v4.is_match(actual.unwrap().as_str()));
    }

    #[tokio::test]
    async fn uuid_modify_with_key() {
        let params = vec!["key"];

        let mut uuid = Uuid::new();
        let actual_first = uuid.modify(params.clone()).await;
        let actual_second = uuid.modify(params.clone()).await;

        let uuid_v4 = Regex::new(UUID_V4_PATTERN).unwrap();

        assert!(uuid_v4.is_match(actual_first.clone().unwrap().as_str()));
        assert_eq!(actual_first, actual_second);
    }
}