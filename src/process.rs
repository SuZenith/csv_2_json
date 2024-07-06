use std::collections::HashMap;
use std::fs;
use csv::Reader;
use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) {
    let mut reader = Reader::from_path(input).unwrap();
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers().unwrap().clone();

    for result in reader.into_records() {
        let record = result.unwrap();
        let mut map = HashMap::new();
        for (header, field) in headers.iter().zip(record.iter()) {
            map.insert(header.to_string(), field.to_string());
        }
        ret.push(map);
    }

    let content: String = match format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(&ret).unwrap()
        },
        OutputFormat::Yaml => {
            serde_yaml::to_string(&ret).unwrap()
        },
        _ => unimplemented!()
    };
    fs::write(output, content).unwrap();
}
