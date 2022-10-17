use anyhow::Result;
use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::matcher::LineMatcher;

pub struct TextFileLineMatcher {
    text_file_path: String,
}

#[async_trait]
impl LineMatcher for TextFileLineMatcher {
    async fn match_line(&self, pattern: String) -> Result<Vec<String>> {
        let file = File::open(&self.text_file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut res = Vec::<String>::new();
        while let Some(line) = lines.next_line().await? {
            if line.contains(pattern.as_str()) {
                res.push(line);
            }
        }

        Ok(res)
    }
}

impl TextFileLineMatcher {
    pub fn new(path: &str) -> Result<TextFileLineMatcher> {
        Ok(TextFileLineMatcher {
            text_file_path: path.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Write};

    use super::*;

    const TEXT_CONTENT: &str = r#"Hello World
File Line Matcher
Text End"#;

    #[tokio::test]
    async fn should_open_file() {
        // given a new text file
        let path = "should_open_file";
        let txt = std::fs::File::create(path).unwrap();
        BufWriter::new(txt)
            .write_all(TEXT_CONTENT.as_bytes())
            .unwrap();

        // when
        let matcher = TextFileLineMatcher::new(path);

        // then
        assert!(matcher.is_ok());
        tokio::fs::remove_file(path).await.unwrap()
    }

    #[tokio::test]
    async fn should_match_line_with_given_phrase() {
        // given a new text file
        let path = "should_match_line_with_given_phrase";
        let txt = std::fs::File::create(path).unwrap();
        BufWriter::new(txt)
            .write_all(TEXT_CONTENT.as_bytes())
            .unwrap();

        // when
        let matcher = TextFileLineMatcher::new(path).unwrap();
        let vec = matcher.match_line("Matcher".into()).await.unwrap();

        // then
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], "File Line Matcher");
        tokio::fs::remove_file(path).await.unwrap()
    }
}
