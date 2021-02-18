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