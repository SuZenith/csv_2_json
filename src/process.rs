use std::collections::HashMap;
use std::fs;
use csv::Reader;
use serde_json::Value;

pub fn process_csv(input: &str, output: &str) {
    let mut reader = Reader::from_path(input).unwrap();
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers().unwrap().clone();

    for result in reader.into_records() {
        let record = result.unwrap();
        let mut map = HashMap::new();

        for (header, field) in headers.iter().zip(record.iter()) {
            map.insert(header.to_string(), field.to_string());
        }

        let json_value: Value = serde_json::to_value(map).unwrap();
        ret.push(json_value);
    }
    fs::write(output, serde_json::to_string_pretty(&ret).unwrap()).unwrap();
}
