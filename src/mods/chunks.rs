use crate::mods::Modifier;

pub const NAME: &str = "chunks";

/// Chunks API responses by providing iterator variables
pub struct Chunks {}

impl Chunks {
    pub fn new() -> Self {
        Chunks {}
    }
}

#[async_trait::async_trait]
impl Modifier for Chunks {
    fn key(&self) -> &'static str { NAME }

    async fn modify(&mut self, _: Vec<&str>) -> Option<String> {
        Some(String::from("todo!")) // fixme
    }
}