use std::fs;
use clap::{Parser};
use csv::{Reader};
use serde::{Deserialize, Serialize};
use csv2json::{Opts, SubCommand};


#[derive(Debug, Deserialize, Serialize)]
struct Record {
    image: String,
    code: String,
    hash: String,
}

// csv2json csv -i input.csv -o output.json
fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input).unwrap();
            let records = reader
                .deserialize()
                .map(|record| record.unwrap())
                .collect::<Vec<Record>>();
            let json = serde_json::to_string_pretty(&records).unwrap();
            fs::write(opts.output, json).unwrap();
        }
    }
}