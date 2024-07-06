use clap::{Parser};
use csv2json::{Opts, process_csv, SubCommand};

// csv2json csv -i input.csv -o output.json
fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(v) = opts.output {
                v.clone()
            } else {
                panic!("Output file is required")
            };
            process_csv(&opts.input, output, opts.format);
        }
    }
}