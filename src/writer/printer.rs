use colored::Colorize;
use crate::matcher::MatchedLine;
use crate::writer::Writer;

pub struct ColoredPrinter {}

impl Writer for ColoredPrinter{
    fn write_all(relative_path: String, lines: Vec<MatchedLine>) -> anyhow::Result<()> {
        println!("{}:", relative_path.green());
        for matched in lines {
            let chars = matched.content.chars().collect::<Vec<_>>();
            println!("    {}:{} {}{}{}",
                     format!("{}", matched.line_no).blue(),
                     format!("{:<4}", matched.first_word_start).blue(),
                     format!("{}", chars[0..matched.first_word_start].into_iter().collect::<String>()),
                     format!("{}", chars[matched.first_word_start..matched.first_word_end].into_iter().collect::<String>()).red(),
                     format!("{}", chars[matched.first_word_end..chars.len()].into_iter().collect::<String>()),
            )
        }

        Ok(())
    }
}