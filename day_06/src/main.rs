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
    let tmp: Vec<HashSet<char>> = group_strings
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

    dbg!(tmp.iter().map(|set| set.len()).sum::<usize>());
}
