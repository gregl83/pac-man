use crate::mods::Modifier;

pub const NAME: &str = "chunks";

/// Chunks API responses by providing iterator variables
pub struct Chunks {
    start: u64,
    chunk_length: u64,
    end: Option<u64>,
    chunk_start: u64,
    chunk_end: u64,
    chunk_index: u64,
    bytes: String
}

impl Chunks {
    pub fn new(start: u64, chunk_length: u64, end: Option<u64>, bytes: &str) -> Self {
        let chunk_end = match end {
            Some(end) => {
                let mut chunk_end = start + chunk_length;
                if chunk_end > end { chunk_end = end; }
                chunk_end
            }
            _ => start + chunk_length
        };

        Chunks {
            start,
            chunk_length,
            end,
            chunk_start: start,
            chunk_end,
            chunk_index: 0,
            bytes: String::from(bytes)
        }
    }
}

#[async_trait::async_trait]
impl Modifier for Chunks {
    fn key(&self) -> &'static str { NAME }

    fn option(&self, key: &str) -> Option<String> {
        match key {
            "bytes" => Some(self.bytes.clone()),
            _ => None
        }
    }

    fn modify(&mut self, params: Vec<&str>) -> Option<String> {
        if params[0].eq("chunk") {
            match params[1] {
                "start" => {
                    return Some(format!("{}", self.chunk_start));
                }
                "end" => {
                    return Some(format!("{}", self.chunk_end));
                }
                "index" => {
                    return Some(format!("{}", self.chunk_index));
                },
                "page" => {
                    return Some(format!("{}", self.chunk_index + 1));
                }
                _ => return None
            }
        }
        None
    }

    fn advance(&mut self) {
        self.chunk_start += self.chunk_length;
        self.chunk_end += self.chunk_length;

        if let Some(end) = self.end {
            if self.chunk_start > end {
                self.chunk_start = end;
            }
            if self.chunk_end > end {
                self.chunk_end = end;
            }
        }

        self.chunk_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn chunks_option_unknown() {
        let start = 0;
        let chunk_length = 10;
        let end = Some(100);
        let bytes = "0";

        let expect = None;

        let chunks = Chunks::new(start, chunk_length, end, bytes);
        let actual = chunks.option("unknown");

        assert_eq!(actual, expect);
    }

    #[tokio::test]
    async fn chunks_option_bytes() {
        let start = 0;
        let chunk_length = 10;
        let end = Some(100);
        let bytes = "0";

        let expect = Some(String::from(bytes.clone()));

        let chunks = Chunks::new(start, chunk_length, end, bytes);
        let actual = chunks.option("bytes");

        assert_eq!(actual, expect);
    }

    #[tokio::test]
    #[should_panic]
    async fn chunks_modify_no_param() {
        let start = 0;
        let chunk_length = 10;
        let end = Some(100);
        let bytes = "0";

        let expect = None;

        let mut chunks = Chunks::new(start, chunk_length, end, bytes);

        let actual = chunks.modify(vec![]);

        assert_eq!(actual, expect);
    }

    #[tokio::test]
    async fn chunks_modify_unknown_param() {
        let start = 0;
        let chunk_length = 10;
        let end = Some(100);
        let bytes = "0";

        let expect = None;

        let mut chunks = Chunks::new(start, chunk_length, end, bytes);

        let actual = chunks.modify(vec!["unknown"]);

        assert_eq!(actual, expect);
    }

    #[tokio::test]
    async fn chunks_modify_params() {
        let start = 0;
        let chunk_length = 10;
        let end = Some(100);
        let bytes = "0";

        let mut chunks = Chunks::new(start, chunk_length, end, bytes);

        let actual_start = chunks.modify(vec!["chunk", "start"]);
        let actual_end = chunks.modify(vec!["chunk", "end"]);
        let actual_index = chunks.modify(vec!["chunk", "index"]);
        let actual_page = chunks.modify(vec!["chunk", "page"]);

        assert_eq!(actual_start, Some(String::from("0")));
        assert_eq!(actual_end, Some(String::from("10")));
        assert_eq!(actual_index, Some(String::from("0")));
        assert_eq!(actual_page, Some(String::from("1")));
    }

    #[tokio::test]
    async fn chunks_advance_to_round_end() {
        let start = 0;
        let chunk_length = 10;
        let end = Some(100);
        let bytes = "0";

        let mut chunks = Chunks::new(start, chunk_length, end, bytes);

        let mut iteration_count = 0;
        let expect_iteration_count = 10;
        loop {
            if chunks.chunk_start >= chunks.chunk_end { break; }
            iteration_count += 1;
            chunks.advance();
        }

        assert_eq!(iteration_count, expect_iteration_count);
        assert_eq!(chunks.chunk_start, 100);
        assert_eq!(chunks.chunk_end, 100);
    }

    #[tokio::test]
    async fn chunks_advance_to_unround_end() {
        let start = 0;
        let chunk_length = 10;
        let end = Some(105);
        let bytes = "0";

        let mut chunks = Chunks::new(start, chunk_length, end, bytes);

        let mut iteration_count = 0;
        let expect_iteration_count = 11;
        loop {
            if chunks.chunk_start >= chunks.chunk_end { break; }
            iteration_count += 1;
            chunks.advance();
        }

        assert_eq!(iteration_count, expect_iteration_count);
        assert_eq!(chunks.chunk_start, 105);
        assert_eq!(chunks.chunk_end, 105);
    }
}