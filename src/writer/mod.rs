use crate::matcher::MatchedLine;
use anyhow::Result;
pub mod printer;

pub trait Writer {
    fn write_all(lines: Vec<MatchedLine>) -> Result<()>;
}