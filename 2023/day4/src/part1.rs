use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Card {
        let winning_numbers = line
            .split("|")
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .filter(|x| *x != "")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let your_numbers = line
            .split("|")
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .filter(|x| *x != "")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        Card {
            winning_numbers: winning_numbers,
            your_numbers: your_numbers,
        }
    }

    fn score(&self) -> u32 {
        let mut sum = 0;
        for number in &self.your_numbers {
            if self.winning_numbers.contains(&number) {
                if sum == 0 {
                    sum = 1;
                } else {
                    sum *= 2;
                }
            }
        }
        sum
    }
}

fn get_input() -> String {
    let mut file = File::open("src/input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    contents
}

pub fn run() {
    let input = get_input();
    let mut total_score = 0;
    for line in input.lines() {
        let card = Card::new(line);
        let score = card.score();
        total_score += score;
    }
    println!("Total score: {}", total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let card = Card::new("B: 1 2 3 4 5 6 7 8 9 10 | 1 2 3 4 5 6 7 8 9 10");
        assert_eq!(card.score(), 512);
        let card = Card::new("B: 1 2 3 4 5 6 7 8 9 10 | 1 11 11 11 11 11 11 11 11 11");
        assert_eq!(card.score(), 1);
        let card = Card::new("B: 1 2 3 4 5 6 7 8 9 10 | 11 11 11 11 11 11 11 11 11 11");
        assert_eq!(card.score(), 0);
    }
}
