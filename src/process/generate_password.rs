use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use anyhow::{Result, anyhow};

pub fn process_generate_password(length: u8, upper: bool, lower: bool, number: bool, symbol: bool) -> Result<String> {
    let upper_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lower_chars = "abcdefghijklmnopqrstuvwxyz";
    let number_chars = "0123456789";
    let symbol_chars = "!@#$%^&*()-_=+[]{}|;:,.<>?";

    let mut char_set = String::new();

    let mut required_chars = Vec::new();

    if upper {
        char_set.push_str(upper_chars);
        required_chars.push(upper_chars.chars().nth(thread_rng().gen_range(0..upper_chars.len())).unwrap());
    }
    if lower {
        char_set.push_str(lower_chars);
        required_chars.push(lower_chars.chars().nth(thread_rng().gen_range(0..lower_chars.len())).unwrap());
    }
    if number {
        char_set.push_str(number_chars);
        required_chars.push(number_chars.chars().nth(thread_rng().gen_range(0..number_chars.len())).unwrap());
    }
    if symbol {
        char_set.push_str(symbol_chars);
        required_chars.push(symbol_chars.chars().nth(thread_rng().gen_range(0..symbol_chars.len())).unwrap());
    }

    if char_set.is_empty() {
        return Err(anyhow!("At least one character type must be selected"));
    }

    if length < required_chars.len() as u8 {
        return Err(anyhow!("Password length is too short to include all required character types"));
    }

    let mut rng = thread_rng();

    let mut password: Vec<char> = required_chars;

    for _ in password.len()..length as usize {
        let idx = rng.gen_range(0..char_set.len());
        password.push(char_set.chars().nth(idx).unwrap());
    }

    password.shuffle(&mut rng);
    println!("{:?}", password.iter().collect::<String>());

    Ok(password.iter().collect())
}