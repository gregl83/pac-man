use uuid;

use crate::mods::Modifier;

pub const NAME: &str = "uuid";

/// Uuid generates uuid v4
pub struct Uuid {}

impl Uuid {
    pub fn new() -> Self {
        Uuid {}
    }
}

#[async_trait::async_trait]
impl Modifier for Uuid {
    fn key(&self) -> &'static str { NAME }

    async fn modify(&mut self, _: Vec<&str>) -> Option<String> {
        Some(uuid::Uuid::new_v4().to_string())
    }
}