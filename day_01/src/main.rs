use std::fs;
use itertools::Itertools;

fn compute(numbers: Vec<usize>) -> Option<(usize, usize)> {
    for arr in numbers.into_iter().permutations(2) {
        if arr[0] + arr[1] == 2020 {
            return Some((arr[0], arr[1]));
        }
    }
    None
}

fn main() {
    let input_str = fs::read_to_string("./src/input.txt")
        .expect("Something went wrong reading the file...");
    let numbers: Vec<usize> = input_str.split("\n")
        .flat_map(|number_str| number_str.parse::<usize>())
        .collect();
    match compute(numbers) {
        Some((a, b)) => println!("{} * {} = {}",a ,b , a*b),
        None => println!("Did not find anything /o\\")
    }
}
