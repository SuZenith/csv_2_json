mod process;
mod cli;

pub use cli::{CsvOpts, Base64SubCommand, Opts, SubCommand};
pub use process::{process_csv, process_generate_password, process_encode, process_decode};