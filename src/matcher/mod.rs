use async_trait::async_trait;
use anyhow::Result;

pub mod file_matcher;

#[async_trait]
pub trait LineMatcher {
    async fn match_line(&self, pattern: String) -> Result<Vec<MatchedLine>>;
}

pub struct MatchedLine {
    pub content: String,
    pub line_no: usize,
    pub first_word_start: usize,
    pub first_word_end: usize
}