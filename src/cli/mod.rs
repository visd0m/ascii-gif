use crate::ascii::symbol::AsciiSymbolEncoding;
use serde::export::Formatter;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(long, required_unless = "id")]
    pub q: Option<String>,
    #[structopt(long, required_unless = "q")]
    pub id: Option<String>,
    #[structopt(short, long)]
    pub tenor: bool,
    #[structopt(short, long)]
    pub giphy: bool,
    #[structopt(long, default_value = "10")]
    pub encoding: AsciiSymbolEncoding,
}

#[derive(Debug)]
pub enum CliError {
    WrongParameters,
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
