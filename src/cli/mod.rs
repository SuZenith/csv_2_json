mod csv;
mod password;
mod base64;
mod text;

use std::path::Path;
use clap::Parser;
pub use csv::{CsvOpts, OutputFormat};
pub use password::PasswordOpts;
pub use base64::{Base64SubCommand, Base64Format};
pub use text::{TextSubCommand, TextSignFormat};

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

    #[command(subcommand)]
    Text(TextSubCommand)
}
pub(super) fn verify_file(filename: &str) -> Result<String, String> {
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
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File not found: *".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File not found: not-exist".into()));
    }
}
