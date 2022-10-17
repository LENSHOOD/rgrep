use async_trait::async_trait;
use anyhow::Result;

pub mod file_matcher;

#[async_trait]
pub trait LineMatcher {
    async fn match_line(&self, pattern: String) -> Result<Vec<String>>;
}