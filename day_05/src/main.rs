use std::fs;

#[derive(Debug)]
struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn seat_id(self) -> usize {
        (self.row as usize * 8 + self.column as usize) as usize
    }
}

fn boarding_pass_to_seat(boarding_pass: String) -> Seat {
    let mut row: u8 = 0;
    let mut column: u8 = 0;
    for (i, character) in boarding_pass[..7].chars().rev().enumerate() {
        match character {
            'B' => {
                row |= 2_u8.pow((i) as u32);
            }
            'F' => {}
            _ => panic!("Character not allowed!"),
        }
    }
    for (i, character) in boarding_pass[7..].chars().rev().enumerate() {
        match character {
            'R' => {
                column |= 2_u8.pow((i) as u32);
            }
            'L' => {}
            _ => panic!("Character not allowed!"),
        }
    }
    return Seat { row, column };
}

fn gauss_sum(n: usize) -> usize {
    (n.pow(2) + n)/2
}

fn main() {
    let input_str =
        fs::read_to_string("./src/input.txt").expect("Something went wrong reading the file...");
    let boarding_passes: Vec<String> = input_str
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();
    let seat_id = boarding_passes
        .iter()
        .map(|pass| boarding_pass_to_seat(pass.to_string()))
        .map(|pass| pass.seat_id());
    let max = seat_id.clone().max().unwrap();
    let min = seat_id.clone().min().unwrap();
    let missing_id = (gauss_sum(max) - gauss_sum(min-1)) - seat_id.sum::<usize>();
    println!("The maximum seat id is {}", max);
    println!("The missing seat id is {}", missing_id);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss() -> Result<(), String> {
        let n: usize = 4;
        assert_eq!(gauss_sum(n), (0..=n).sum());
        Ok(())
    }

    #[test]
    fn test_subtraction() -> Result<(), String> {
        assert_eq!(gauss_sum(10) - gauss_sum(5-1), (5..=10).sum());
        Ok(())
    }
}

