use std::fs;

#[derive(Debug)]
struct Steps {
    right: usize,
    down: usize
}

#[derive(Debug)]
struct Traversal {
    geology: Vec<Vec<char>>,
    position: (usize, usize),
    steps: Steps
}

impl Iterator for Traversal {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.position.0 + self.steps.down >= self.geology.len() {
            return None
        }
        self.position.0 = self.position.0 + self.steps.down;
        self.position.1 = (self.position.1 + self.steps.right) % (self.geology[0].len());
        Some(self.geology[self.position.0][self.position.1])
    }
}

fn main() {
    let input_str = fs::read_to_string("./src/input.txt")
        .expect("Something went wrong reading the file...");
    let input_lines: Vec<Vec<char>> = input_str.split('\n').map(String::from).filter(|x| !x.is_empty()).map(|x| x.chars().collect()).collect();

    let mut results: Vec<usize> = Vec::new();
    results.push(
        Traversal {
            geology: input_lines.clone(),
            position: (0,0),
            steps: Steps { right: 1, down: 1 }
        }.into_iter().filter(|x| *x == '#').collect::<Vec<char>>().len());
    results.push(
        Traversal {
            geology: input_lines.clone(),
            position: (0,0),
            steps: Steps { right: 3, down: 1 }
        }.into_iter().filter(|x| *x == '#').collect::<Vec<char>>().len());
    results.push(
        Traversal {
            geology: input_lines.clone(),
            position: (0,0),
            steps: Steps { right: 5, down: 1 }
        }.into_iter().filter(|x| *x == '#').collect::<Vec<char>>().len());
    results.push(
        Traversal {
            geology: input_lines.clone(),
            position: (0,0),
            steps: Steps { right: 7, down: 1 }
        }.into_iter().filter(|x| *x == '#').collect::<Vec<char>>().len());
    results.push(
        Traversal {
            geology: input_lines.clone(),
            position: (0,0),
            steps: Steps { right: 1, down: 2 }
        }.into_iter().filter(|x| *x == '#').collect::<Vec<char>>().len());

    let result = results.iter().fold(1, |agg, i| agg * i);
    println!("{}", result);

}
