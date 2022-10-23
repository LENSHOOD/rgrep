use async_trait::async_trait;
use anyhow::Result;

pub mod file_matcher;

#[async_trait]
pub trait LineMatcher {
    async fn match_line(&self, pattern: String) -> Result<Vec<MatchedLine>>;
}

pub struct MatchedLine {
    pub content: String,
    pub line_no: u64,
    pub first_word_start: u64,
    pub first_word_end: u64
}