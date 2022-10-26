use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use anyhow::Result;
use async_trait::async_trait;
use regex::{Match, Regex};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::task::JoinHandle;

use crate::matcher::{LineMatcher, MatchedLine};

pub struct TextFileLineMatcher {
    text_file_path: String,
}

#[async_trait]
impl LineMatcher for TextFileLineMatcher {
    async fn match_line(&self, pattern: String) -> Result<Vec<MatchedLine>> {
        let file = File::open(&self.text_file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut res = Vec::<MatchedLine>::new();
        let line_cnt = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::<JoinHandle<Result<Option<MatchedLine>>>>::new();
        while let Some(line) = lines.next_line().await? {
            let l = line.clone();
            let p = pattern.clone();
            let lc = line_cnt.clone();
            let handle: JoinHandle<Result<Option<MatchedLine>>> = tokio::task::spawn_blocking(move || {
                let match_result = match_regex(p.as_str(), line.as_str())?;
                if match_result.is_some() {
                    return Ok(Some(MatchedLine {
                        content: l.clone(),
                        line_no: lc.clone().fetch_add(1, Ordering::SeqCst),
                        first_word_start: match_result.unwrap().start(),
                        first_word_end: match_result.unwrap().end(),
                    }))
                }

                Ok(None)
            });

            handles.push(handle);
        }

        while let Some(handle) = handles.pop() {
            let matched_lines = handle.await??;
            if matched_lines.is_some() {
                res.push(matched_lines.unwrap());
            }
        }

        Ok(res)
    }
}

fn match_regex<'a>(pattern: &str, to_be_matched: &'a str) -> Result<Option<Match<'a>>> {
    let regex = Regex::new(pattern)?;
    Ok(regex.find(to_be_matched))
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
        assert_eq!(vec[0].content, "File Line Matcher");
        assert_eq!(vec[0].line_no, 3);
        assert_eq!(vec[0].first_word_start, 10);
        assert_eq!(vec[0].first_word_end, 17);
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
        let pattern = r"([\d]+)|([\[].+[\]])|([\u4e00-\u9fa5])";
        let vec = matcher.match_line(pattern.into()).await.unwrap();

        // then
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0].content, "Numbers: 1, 2, 3.456");
        assert_eq!(vec[0].line_no, 2);
        assert_eq!(vec[0].first_word_start, 9);
        assert_eq!(vec[0].first_word_end, 10);
        assert_eq!(vec[1].content, "多语言");
        assert_eq!(vec[1].line_no, 4);
        assert_eq!(vec[1].first_word_start, 0);
        assert_eq!(vec[1].first_word_end, 3);
        assert_eq!(vec[2].content, "Match Brackets: [in_the_bracket1233456]");
        assert_eq!(vec[2].line_no, 5);
        assert_eq!(vec[2].first_word_start, 16);
        assert_eq!(vec[2].first_word_end, 39);
        tokio::fs::remove_file(path).await.unwrap()
    }
}
