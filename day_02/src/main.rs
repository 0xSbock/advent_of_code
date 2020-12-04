use std::str::FromStr;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;


#[derive(Debug)]
struct CorruptedPassword {
    min_occurrence: usize,
    max_occurrence: usize,
    character: char,
    password: String
}

impl FromStr for CorruptedPassword {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = s.split_whitespace();
        let mut occurrence_counts = elements.next().ok_or("empty input")?.split("-");
        let min_occurrence = occurrence_counts.next().ok_or("min count is missing")?.parse::<usize>()?;
        let max_occurrence = occurrence_counts.next().ok_or("min count is missing")?.parse::<usize>()?;
        let character: char = elements.next().ok_or("character is missing")?.chars().next().unwrap();
        let password: String = elements.next().ok_or("password is missing")?.to_string();
        Ok(CorruptedPassword{
            min_occurrence: min_occurrence,
            max_occurrence: max_occurrence,
            character: character,
            password: password
        })
    }
}

impl CorruptedPassword {
    fn is_uncorrupted(&self) -> bool {
        let char_count = self.password.chars().filter(|&c| c == self.character).collect::<Vec<char>>().len();
        (self.min_occurrence..=self.max_occurrence).contains(&char_count)
    }
}


fn main() -> io::Result<()> {
    let f = File::open("./src/input.txt")?;
    let f = BufReader::new(f);

    let mut uncorrupted_count = 0;
    for line in f.lines() {
        match CorruptedPassword::from_str(&line?) {
            Ok(cpwd) => {
                if cpwd.is_uncorrupted() {
                    uncorrupted_count += 1;
                }
            },
            // break if input is empty on from string method
            Err(_) => break
        }
    }
    println!("{}", uncorrupted_count);
    Ok(())
}
