use colored::Colorize;
use crate::matcher::MatchedLine;
use crate::writer::Writer;

pub struct ColoredPrinter {}

impl Writer for ColoredPrinter{
    fn write_all(lines: Vec<MatchedLine>) -> anyhow::Result<()> {
        for matched in lines {
            println!("{}:{} {}",
                     format!("{}", matched.line_no).blue(),
                     format!("{}", matched.word_idx).blue(),
                     format!("{}", matched.content)
            )
        }

        Ok(())
    }
}