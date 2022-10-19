use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
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
            if is_match_regex(pattern.as_str(), line.as_str())? {
                res.push(line);
            }
        }

        Ok(res)
    }
}

fn is_match_regex(pattern: &str, to_be_matched: &str) -> Result<bool> {
    let regex = Regex::new(pattern)?;
    Ok(regex.is_match(to_be_matched))
}

impl TextFileLineMatcher {
    pub async fn new(path: &str) -> Result<TextFileLineMatcher> {
        let _ = File::open(path).await?;
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
Numbers: 1, 2, 3.456
File Line Matcher
多语言
Match Brackets: [in_the_bracket1233456]
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
        let matcher = TextFileLineMatcher::new(path).await;

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
        let matcher = TextFileLineMatcher::new(path).await.unwrap();
        let vec = matcher.match_line("Matcher".into()).await.unwrap();

        // then
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], "File Line Matcher");
        tokio::fs::remove_file(path).await.unwrap()
    }

    #[tokio::test]
    async fn should_match_line_with_regex() {
        // given a new text file
        let path = "should_match_line_with_regex";
        let txt = std::fs::File::create(path).unwrap();
        BufWriter::new(txt)
            .write_all(TEXT_CONTENT.as_bytes())
            .unwrap();

        // when
        let matcher = TextFileLineMatcher::new(path).await.unwrap();
        let pattern = r"([\d]+)|([\[].+[\]])";
        let vec = matcher.match_line(pattern.into()).await.unwrap();

        // then
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], "Numbers: 1, 2, 3.456");
        assert_eq!(vec[1], "Match Brackets: [in_the_bracket1233456]");
        tokio::fs::remove_file(path).await.unwrap()
    }
}
