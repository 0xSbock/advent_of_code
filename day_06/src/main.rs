use std::collections::HashSet;
use std::fs;



fn main() {
    let input_str =
        fs::read_to_string("./src/input.txt").expect("Something went wrong reading the file...");
    let group_strings = input_str
        .split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .filter(|x| x.len() > 0)
        .collect::<Vec<Vec<String>>>();
    let stage_1: Vec<HashSet<char>> = group_strings
        .clone()
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|answer| answer.chars().collect::<HashSet<char>>())
                .fold(HashSet::<char>::new(), |a, b| {
                    a.union(&b).map(|&c| c).collect::<HashSet<char>>()
                })
        })
        .collect();
    let stage_2: Vec<HashSet<char>> = group_strings
        .clone()
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|answer| answer.chars().collect::<HashSet<char>>())
                .fold(('a'..'z').collect::<HashSet<char>>(), |a, b| {
                    a.intersection(&b).map(|&c| c).collect::<HashSet<char>>()
                })
        })
        .collect();

    dbg!(stage_1.iter().map(|set| set.len()).sum::<usize>());
    dbg!(stage_2.iter().map(|set| set.len()).sum::<usize>());
}
