use clap::{Parser};
use zenith::{Base64SubCommand, Opts, process_csv, process_decode, process_encode, process_generate_password, SubCommand};

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(v) = opts.output {
                v.clone()
            } else {
                format!("{}.json", opts.input)
            };
            process_csv(&opts.input, output, opts.format);
        }
        SubCommand::Password(opts) => {
            process_generate_password(opts.length as u8, opts.uppercase, opts.lowercase, opts.number, opts.symbol).unwrap();
        }
        SubCommand::Base64(base64_sub_command) => match base64_sub_command {
            Base64SubCommand::Encode(opts) => {
                let _ = process_encode(&opts.input, opts.format);
            }
            Base64SubCommand::Decode(opts) => {
                let _ = process_decode(&opts.input, opts.format);
            }
        }
    }
}