mod process;
mod cli;

pub use cli::opts::{Opts, SubCommand};
pub use process::{process_csv, process_generate_password};