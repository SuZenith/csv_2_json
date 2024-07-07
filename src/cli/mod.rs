mod csv;
mod password;
mod base64;

use std::path::Path;
use clap::Parser;
pub use csv::{CsvOpts, OutputFormat};
pub use password::PasswordOpts;
pub use base64::{Base64SubCommand, Base64Format};

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

    #[command(subcommand)]
    Base64(Base64SubCommand),
}
pub(super) fn verify_input_file(filename: &str) -> Result<String, String> {
    // if input is "-" or file exits
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File not found: {}", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File not found: *".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File not found: not-exist".into()));
    }
}
