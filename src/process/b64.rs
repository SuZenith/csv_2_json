use std::fs::File;
use std::io::{Read, stdin};
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::prelude::*;
use crate::cli::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<(), anyhow::Error> {
    let mut reader = get_reader(input);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<(), anyhow::Error> {
    let mut reader = get_reader(input);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf),
    };

    println!("{}", String::from_utf8(decoded.unwrap())?);
    Ok(())
}

fn get_reader(input: &str) -> Box<dyn Read> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        Box::new(File::open(input).unwrap())
    };
    reader
}

mod tests {

    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }
}
