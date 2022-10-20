mod matcher;

use std::process::exit;
use clap::Parser;
use crate::matcher::file_matcher::TextFileLineMatcher;
use crate::matcher::LineMatcher;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct GrepArgs {
    // pattern for string to be matched
    #[arg(short, long)]
    pattern: String,

    // file to be grep
    #[arg(short, long)]
    file: String
}

#[tokio::main]
async fn main() {
    let args = GrepArgs::parse();
    let line_matcher = TextFileLineMatcher::new(args.file.as_ref()).await;
    if line_matcher.is_err() {
        println!("Err: {}", line_matcher.err().unwrap());
        exit(1);
    }

    let result = line_matcher.unwrap().match_line(args.pattern).await;
    if result.is_err() {
        println!("Err: {}", result.err().unwrap());
        exit(1);
    }

    for matched in result.unwrap() {
        println!("{}:{} {}", matched.line_no, matched.word_idx, matched.content)
    }
}
