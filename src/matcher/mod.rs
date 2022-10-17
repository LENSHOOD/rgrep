use async_trait::async_trait;

mod file_matcher;

#[async_trait]
pub trait LineMatcher {
    async fn match_line(pattern: String) -> Vec<String>;
}