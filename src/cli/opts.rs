use clap::Parser;
use crate::cli::{CsvOpts, PasswordOpts};

#[derive(Debug, Parser)]
#[command(name = "csv2json", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),

    #[command(name = "password", about = "Generate a random password")]
    Password(PasswordOpts),
}

