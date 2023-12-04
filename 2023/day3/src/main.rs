use core::num;
use std::char;
use std::fs::File;
use std::io::prelude::*;

fn is_dot(c: char) -> bool {
    c == '.'
}

#[derive(Debug)]
struct Input {
    raw: String,
    matrix: Vec<Vec<char>>,
    position_x: i32,
    position_y: i32,
}

impl Input {
    fn compare(&mut self, number: &NumberPosition, symbols: &Vec<SymbolPosition>) -> bool {
        for symbol in symbols {
            let range = number.start_x - 1..number.end_x + 1;
            for x in range {
                if symbol.y == number.y && symbol.x == x {
                    return true;
                } else if symbol.y == number.y - 1 && symbol.x == x {
                    return true;
                } else if symbol.y == number.y + 1 && symbol.x == x {
                    return true;
                }
            }
        }
        false
    }

    fn parse(&mut self) -> PositionCollection {
        let mut number_positions = Vec::new();
        let mut symbol_positions = Vec::new();

        loop {
            let line = match self.matrix.get(self.position_y as usize) {
                Some(line) => line,
                None => break,
            };
            self.position_x = 0;
            loop {
                let char = match line.get(self.position_x as usize) {
                    Some(char) => char,
                    None => break,
                };
                if is_dot(*char) {
                    self.position_x += 1;
                    continue;
                } else if char.is_numeric() {
                    let start_x = self.position_x.clone();
                    let mut end_x = self.position_x.clone();
                    let mut number = Vec::new();
                    number.push(*char);
                    loop {
                        self.position_x += 1;
                        let char = match line.get(self.position_x as usize) {
                            Some(char) => char,
                            None => break,
                        };
                        if char.is_numeric() {
                            number.push(*char);
                        } else {
                            end_x = self.position_x.clone();
                            break;
                        }
                    }
                    let number: i32 = number.iter().collect::<String>().parse().unwrap();
                    number_positions.push(NumberPosition {
                        y: self.position_y,
                        start_x: start_x,
                        end_x: end_x,
                        number: number,
                    });
                    continue;
                } else {
                    symbol_positions.push(SymbolPosition {
                        y: self.position_y,
                        x: self.position_x,
                    });
                }
                self.position_x += 1;
            }
            self.position_y += 1;
        }

        PositionCollection {
            numbers: number_positions,
            symbols: symbol_positions,
        }
    }

    fn get_char_at(&self, x: i32, y: i32) -> Result<char, ()> {
        let line = match self.matrix.get(y as usize) {
            Some(line) => line,
            None => return Err(()),
        };
        let char = match line.get(x as usize) {
            Some(char) => char,
            None => return Err(()),
        };
        Ok(*char)
    }
}

#[derive(Debug)]
struct SymbolPosition {
    y: i32,
    x: i32,
}

#[derive(Debug)]
struct NumberPosition {
    y: i32,
    start_x: i32,
    end_x: i32,
    number: i32,
}

#[derive(Debug)]
struct PositionCollection {
    numbers: Vec<NumberPosition>,
    symbols: Vec<SymbolPosition>,
}

fn get_input() -> Input {
    let mut file = File::open("src/input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    let mut matrix = Vec::new();
    for line in contents.lines() {
        let mut line_vec = Vec::new();
        for char in line.chars() {
            line_vec.push(char);
        }
        matrix.push(line_vec);
    }

    Input {
        raw: contents,
        matrix: matrix,
        position_x: 0,
        position_y: 0,
    }
}

fn main() {
    let mut input = get_input();
    let collection = input.parse();
    let mut sum = 0;
    for number in collection.numbers {
        let is_valid = input.compare(&number, &collection.symbols);
        println!("{:?}{}", number, is_valid);
        if is_valid {
            sum += number.number;
        }
    }
    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_char_at() {
        let input = Input {
            raw: String::from("12..23..34\n42..53..64\n72..83..94"),
            matrix: vec![
                vec!['1', '2', '.', '.', '2', '3', '.', '.', '3', '4'],
                vec!['4', '2', '.', '.', '5', '3', '.', '.', '6', '4'],
                vec!['7', '2', '.', '.', '8', '3', '.', '.', '9', '4'],
            ],
            position_x: 0,
            position_y: 0,
        };
        assert_eq!(input.get_char_at(0, 0), Ok('1'));
        assert_eq!(input.get_char_at(1, 0), Ok('2'));
        assert_eq!(input.get_char_at(4, 0), Ok('2'));
        assert_eq!(input.get_char_at(8, 2), Ok('9'));
        assert_eq!(input.get_char_at(9, 2), Ok('4'));
        assert_eq!(input.get_char_at(10, 2), Err(()));
        assert_eq!(input.get_char_at(1, 10), Err(()));
    }
}
