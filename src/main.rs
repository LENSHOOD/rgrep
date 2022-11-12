use std::fs;
use std::time::SystemTime;
use anyhow::Result;

use clap::Parser;
use colored::Colorize;

use crate::matcher::file_matcher::TextFileLineMatcher;
use crate::matcher::LineMatcher;
use crate::writer::printer::ColoredPrinter;
use crate::writer::Writer;

mod matcher;
mod writer;

/// rgrep == grep by rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct GrepArgs {
    /// pattern for string to be matched
    pattern: String,

    /// file to be grep, or dir to grep all files under that dir and sub dirs recursively
    path: String,

    /// debug mode to calculate running time elapse
    #[arg(short, long)]
    debug: bool
}

#[tokio::main]
async fn main() {
    let args = GrepArgs::parse();
    let start = SystemTime::now();

    let mut path_container: Vec<String> = vec![args.path];
    while let Some(curr) = path_container.pop() {
        if fs::metadata(curr.clone()).unwrap().is_file() {
            match grep_single_file(curr.clone(), args.pattern.clone()).await {
                Ok(_) => {}
                Err(err) => {println!("Err at {}: {}", curr.clone().green(), err)}
            }
            continue;
        }

        for entry in fs::read_dir(curr.clone()).unwrap() {
            let path = entry.unwrap().path();
            let path_of_string = path.clone().into_os_string().into_string().unwrap();
            path_container.push(path_of_string)
        }
    }

    if args.debug {
        let duration = start.elapsed().unwrap();
        println!("Time elapsed: {} ms", duration.as_millis())
    }
}

async fn grep_single_file(file_path: String, pattern: String) -> Result<()> {
    let line_matcher = TextFileLineMatcher::new(file_path.as_str()).await?;
    let matched_lines = line_matcher.match_line(pattern).await?;
    if matched_lines.is_empty() {
        return Ok(())
    }

    ColoredPrinter::write_all(file_path, matched_lines)
}
