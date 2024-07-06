use clap::{Parser};
use zenith::{Opts, process_csv, process_generate_password, SubCommand};

// csv2json csv -i input.csv -o output.json
fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(v) = opts.output {
                v.clone()
            } else {
                format!("{}.json", opts.input)
            };
            process_csv(&opts.input, output.to_string(), opts.format);
        }
        SubCommand::Password(opts) => {
            process_generate_password(opts.length as u8, opts.uppercase, opts.lowercase, opts.number, opts.symbol).unwrap();
        }
    }
}