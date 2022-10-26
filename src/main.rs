use std::process::exit;
use std::time::SystemTime;

use clap::Parser;

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

    /// file to be grep
    file: String,

    /// debug mode to calculate running time elapse
    #[arg(short, long)]
    debug: bool
}

#[tokio::main]
async fn main() {
    let args = GrepArgs::parse();
    let line_matcher = TextFileLineMatcher::new(args.file.as_ref()).await;
    if line_matcher.is_err() {
        println!("Err: {}", line_matcher.err().unwrap());
        exit(1);
    }

    let start = SystemTime::now();

    let result = line_matcher.unwrap().match_line(args.pattern).await;
    if result.is_err() {
        println!("Err: {}", result.err().unwrap());
        exit(1);
    }

    ColoredPrinter::write_all(result.unwrap())
        .unwrap_or_else(|e| println!("Err: {}", e));

    if args.debug {
        let duration = start.elapsed().unwrap();
        println!("Time elapsed: {} ms", duration.as_millis())
    }
}
