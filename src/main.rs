use clap::{Parser};
use zenith::{Base64SubCommand, Opts, process_csv, process_decode, process_encode, process_generate_password, process_sign, process_verify, SubCommand, TextSubCommand};

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
        SubCommand::Text(text_sub_command) => match text_sub_command {
            TextSubCommand::Sign(opts) => {
                let _ = process_sign(&opts.input, &opts.key, opts.format);
            }
            TextSubCommand::Verify(opts) => {
                let _ = process_verify(&opts.input, &opts.key, &opts.sig, opts.format);
            }
        }
    }
}