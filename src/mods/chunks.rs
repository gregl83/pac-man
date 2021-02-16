use crate::mods::Modifier;

pub const NAME: &str = "chunks";

/// Chunks API responses by providing iterator variables
pub struct Chunks {
    start: usize,
    chunk_length: usize,
    end: Option<usize>,
    chunk_start: usize,
    chunk_end: usize
}

impl Chunks {
    pub fn new(start: usize, chunk_length: usize, end: Option<usize>) -> Self {
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
            chunk_end
        }
    }
}

#[async_trait::async_trait]
impl Modifier for Chunks {
    fn key(&self) -> &'static str { NAME }

    async fn modify(&mut self, params: Vec<&str>) -> Option<String> {
        if params[0].eq("chunk") {
            match params[1] {
                "start" => {
                    return Some(format!("{}", self.chunk_start));
                }
                "end" => {
                    return Some(format!("{}", self.chunk_end));
                }
                "index" => {
                    let index = self.start / self.chunk_length;
                    return Some(format!("{}", index));
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
    }
}