use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
struct Passport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: usize,
    hair_color: String,
    //what color is lzr supposed to be
    eye_color: String,
    passport_id: usize,
    country_id: Option<usize>,
}

fn generate_passport(candidate: Vec<String>) -> Result<()> {
    let mut store: HashMap<String, String> = HashMap::new();
    for key_pair in candidate {
        let key = key_pair.split(":").nth(0).unwrap();
        let value = key_pair.split(":").nth(1).unwrap();
        store.insert(key.to_string(), value.to_string());
    }

    for key in vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        if !store.contains_key(key) {
            return Err(anyhow!("Missing attribute: {}", key));
        }
    }
    Ok(())
}

fn main() {
    let input_str =
        fs::read_to_string("./src/input.txt").expect("Something went wrong reading the file...");
    let candidate_strings: Vec<Vec<String>> = input_str
        .split("\n\n")
        .map(String::from)
        .map(|x| {
            x.split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .filter(|x| x.len() > 0)
        .collect::<Vec<Vec<String>>>();

    let result = candidate_strings
        .iter()
        .map(|x| generate_passport(x.to_vec()))
        .filter(|x| x.is_ok())
        .count();
    dbg!(result);
}
