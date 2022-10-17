use async_trait::async_trait;
use tokio::fs;
use tokio::fs::File;
use anyhow::Result;
use crate::matcher::LineMatcher;

pub struct TextFileLineMatcher {
    text_file: fs::File
}

#[async_trait]
impl LineMatcher for TextFileLineMatcher {
    async fn match_line(pattern: String) -> Vec<String> {
        todo!()
    }
}

impl TextFileLineMatcher {
    pub async fn new(path: &str) -> Result<TextFileLineMatcher> {
        let file = File::open(path).await?;
        Ok(TextFileLineMatcher { text_file: file })
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Write};
    use super::*;

    const TEXT_CONTENT: &str = r#"
        Hello World
        File Line Matcher
        Text End
        "#;

    #[tokio::test]
    async fn should_open_file() {
        // given a new text file
        let path = "test_txt_file";
        let txt = std::fs::File::create(path).unwrap();
        BufWriter::new(txt).write_all(TEXT_CONTENT.as_bytes()).unwrap();

        // when
        let matcher = TextFileLineMatcher::new(path).await;

        // then
        assert!(matcher.is_ok());
        tokio::fs::remove_file(path).await.unwrap()
    }
}