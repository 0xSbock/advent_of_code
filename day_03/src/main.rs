use std::fs;

#[derive(Debug)]
struct Forest {
    geology: Vec<Vec<char>>,
    position: (usize, usize)
}

impl Iterator for Forest {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.position.0 + 1 >= self.geology.len() {
            return None
        }
        self.position.0 = self.position.0 + 1;
        self.position.1 = (self.position.1 + 3) % (self.geology[0].len());
        Some(self.geology[self.position.0][self.position.1])
    }
}

fn main() {
    let input_str = fs::read_to_string("./src/input.txt")
        .expect("Something went wrong reading the file...");
    let input_lines: Vec<Vec<char>> = input_str.split('\n').map(String::from).filter(|x| !x.is_empty()).map(|x| x.chars().collect()).collect();
    let forest = Forest {
        geology: input_lines,
        position: (0,0)
    };
    let trees = forest.into_iter().filter(|x| *x == '#').collect::<Vec<char>>().len();
    println!("While traversing, we encountered {} trees.", trees);

}
