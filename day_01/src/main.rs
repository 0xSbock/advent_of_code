use std::fs;
use itertools::Itertools;

fn compute(numbers: Vec<usize>, number_count: usize) -> Option<Vec<usize>> {
    for arr in numbers.into_iter().permutations(number_count) {
        let current_sum = arr.iter().fold(0, |acc, x| acc + x);
        if current_sum == 2020 {
            return Some(arr);
        }
    }
    None
}

fn generate_output(numbers: Vec<usize>, number_count: usize) {
    match compute(numbers.clone(), number_count) {
        Some(arr) => {
            let formatting = arr.iter().join("*");
            let result = arr.iter().fold(1, |acc, x| acc * x);
            println!("{} = {}", formatting, result);
        },
        None => println!("Did not find anything /o\\")
    }
}

fn main() {
    let input_str = fs::read_to_string("./src/input.txt")
        .expect("Something went wrong reading the file...");
    let numbers: Vec<usize> = input_str.split("\n")
        .flat_map(|number_str| number_str.parse::<usize>())
        .collect();
    print!("Stage 1: ");
    generate_output(numbers.clone(), 2);
    print!("Stage 2: ");
    generate_output(numbers, 3);
}
