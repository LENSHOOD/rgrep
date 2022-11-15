#![feature(async_closure)]

use std::time::SystemTime;
use anyhow::Result;

use clap::Parser;
use colored::Colorize;

use crate::matcher::file_matcher::TextFileLineMatcher;
use crate::matcher::LineMatcher;
use crate::utils::walk_dir;
use crate::writer::printer::ColoredPrinter;
use crate::writer::Writer;

mod matcher;
mod writer;
mod utils;

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

    let pattern = args.pattern.as_str();
    walk_dir(args.path, async move |file_path| {
        match grep_single_file(file_path.clone(), pattern.to_string()).await {
            Ok(_) => {}
            Err(err) => {println!("Err at {}: {}", file_path.green(), err)}
        }
    }).await.unwrap();

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
