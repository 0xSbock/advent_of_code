use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::fs;

enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

enum HeightUnit {
    Cm,
    In,
}

struct Height {
    height: usize,
    unit: HeightUnit,
}

struct Passport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: Height,
    hair_color: String,
    eye_color: EyeColor,
    passport_id: String,
    // country_id: Option<usize>,
}

fn validate_year(year_string: String, bounds: (usize, usize), field: String) -> Result<usize> {
    match year_string.parse::<usize>() {
        Ok(year) => match (bounds.0..=bounds.1).contains(&year) {
            true => Ok(year),
            false => Err(anyhow!("Year {} out of bounds", year)),
        },
        e => e.context(format!("{} year parser error", field)),
    }
}

fn validate_height(height_string: String) -> Result<Height> {
    let height = height_string
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let unit_string = height_string
        .chars()
        .filter(|x| !x.is_digit(10))
        .collect::<String>();
    let unit = match unit_string.as_str() {
        "cm" => HeightUnit::Cm,
        "in" => HeightUnit::In,
        _ => return Err(anyhow!("Wrong height unit!")),
    };

    match unit {
        HeightUnit::Cm => {
            if !(150..=193).contains(&height) {
                return Err(anyhow!("Height out of centimeter bounds"));
            }
        },
        HeightUnit::In => {
            if !(59..=76).contains(&height) {
                return Err(anyhow!("Height out of inches bounds"));
            }
        }
    }
    Ok(Height {
        height: height,
        unit: unit,
    })
}

fn validate_hair_color(hair_string: String) -> Result<String> {
    let mut hair_iter = hair_string.chars();
    if !(hair_iter.next().ok_or("Empty string").unwrap() == '#') {
        return Err(anyhow!("Wrong color format"));
    }
    for character in hair_iter {
        if !(('a'..='f').contains(&character)) && !(('0'..='9').contains(&character)) {
            return Err(anyhow!("Wrong color format"));
        }
    }
    Ok(hair_string)
}

fn validate_eye_color(eye_color: String) -> Result<EyeColor> {
    match eye_color.as_str() {
        "amb" => Ok(EyeColor::Amber),
        "blu" => Ok(EyeColor::Blue),
        "brn" => Ok(EyeColor::Brown),
        "gry" => Ok(EyeColor::Grey),
        "grn" => Ok(EyeColor::Green),
        "hzl" => Ok(EyeColor::Hazel),
        "oth" => Ok(EyeColor::Other),
        _ => Err(anyhow!("Unsupported eye color")),
    }
}

fn validate_passport_id(passport_id: String) -> Result<String> {
    let number_count = passport_id
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>()
        .len();
    if number_count != 9 {
        return Err(anyhow!("Unsupported number of passport id numbers"));
    }
    return Ok(passport_id);
}

fn generate_passport(candidate: Vec<String>) -> Result<Passport> {
    let mut store: HashMap<String, String> = HashMap::new();
    for key_pair in candidate {
        let key = key_pair.split(":").nth(0).unwrap();
        let value = key_pair.split(":").nth(1).unwrap();
        store.insert(key.to_string(), value.to_string());
    }

    let birth_string = match &store.get(&String::from("byr")) {
        None => return Err(anyhow!("Key not in candidate")),
        Some(res) => res.to_string()
    };
    let birth_year = match validate_year(
        birth_string, (1920, 2002), "Birth".to_string()
    ) {
        Ok(year) => year,
        Err(e) => return Err(e),
    };

    let issue_string = match &store.get(&String::from("iyr")) {
        None => return Err(anyhow!("Key not in candidate")),
        Some(res) => res.to_string()
    };
    let issue_year = match validate_year(
        issue_string, (2010, 2020), "Issue".to_string()
    ) {
        Ok(year) => year,
        Err(e) => return Err(e),
    };

    let expiration_string = match &store.get(&String::from("eyr")) {
        None => return Err(anyhow!("Key not in candidate")),
        Some(res) => res.to_string()
    };
    let expiration_year = match validate_year(
        expiration_string, (2020, 2030), "Expiration".to_string()
    ) {
        Ok(year) => year,
        Err(e) => return Err(e),
    };


    let hair_string = match &store.get(&String::from("hcl")) {
        None => return Err(anyhow!("Key not in candidate")),
        Some(res) => res.to_string()
    };
    let hair_color = match validate_hair_color(hair_string) {
        Ok(hair_color) => hair_color,
        Err(e) => return Err(e),
    };

    let height_string = match &store.get(&String::from("hgt")) {
        None => return Err(anyhow!("Key not in candidate")),
        Some(res) => res.to_string()
    };
    let height = match validate_height(height_string) {
        Ok(height) => height,
        Err(e) => return Err(e),
    };

    let eye_string = match &store.get(&String::from("ecl")) {
        None => return Err(anyhow!("Key not in candidate")),
        Some(res) => res.to_string()
    };
    let eye_color = match validate_eye_color(eye_string) {
        Ok(eye_color) => eye_color,
        Err(e) => return Err(e),
    };

    let pid_string = match &store.get(&String::from("pid")) {
        None => return Err(anyhow!("Key not in candidate")),
        Some(res) => res.to_string()
    };
    let passport_id = match validate_passport_id(pid_string) {
        Ok(pid) => pid,
        Err(e) => return Err(e),
    };
 

    Ok(Passport {
        birth_year,
        issue_year,
        expiration_year,
        height,
        hair_color,
        eye_color,
        passport_id,
    })
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
