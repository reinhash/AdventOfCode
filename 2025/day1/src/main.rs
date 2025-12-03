use std::convert::From;
use std::path::Path;

use parser::load_input;

mod parser;

#[derive(Eq, PartialEq, Debug, Clone)]
struct ZeroCount(u16);

impl ZeroCount {
    fn add(&mut self) {
        self.0 += 1;
    }
}

struct Position(i16);

impl Position {
    fn rotate(&mut self, rotation: Rotation, zero_count: &mut ZeroCount) {
        match rotation {
            Rotation::Left(num) => {
                let rotations_to_apply = num % 100;
                let rotations_to_apply = rotations_to_apply as i16;
                if rotations_to_apply > self.0 {
                    self.0 += 100 - rotations_to_apply;
                } else {
                    self.0 -= rotations_to_apply;
                }
            }
            Rotation::Right(num) => {
                let rotations_to_apply = num % 100;
                let rotations_to_apply = rotations_to_apply as i16;
                self.0 = self.0.checked_add(rotations_to_apply).unwrap();
                if self.0 >= 100 {
                    self.0 -= 100;
                }
            }
        }
        if self.0 == 0 {
            zero_count.add();
        }
    }
}

#[derive(Debug, Clone)]
enum Rotation {
    Left(u16),
    Right(u16),
}

impl From<String> for Rotation {
    fn from(value: String) -> Self {
        let prefix = value.chars().nth(0).unwrap();
        let postfix: u16 = value.split_at(1).1.parse().unwrap();
        match prefix {
            'L' => Rotation::Left(postfix),
            'R' => Rotation::Right(postfix),
            // we expect clean input for this exercise
            _ => unreachable!(),
        }
    }
}

fn part_1(rotations: Vec<Rotation>) -> u16 {
    let mut position = Position(50);
    let mut zero_count = ZeroCount(0);
    for rotation in rotations {
        position.rotate(rotation, &mut zero_count);
    }
    zero_count.0
}

fn part_2(rotations: Vec<Rotation>) -> u16 {
    let mut position = Position(50);
    let mut zero_count = ZeroCount(0);
    for rotation in rotations {
        // Spread rotations into steps of 1. Not very pretty but should work.
        let rotation_num = match &rotation {
            Rotation::Left(num) => *num,
            Rotation::Right(num) => *num,
        };
        for _ in 0..rotation_num {
            let single_step_rotation = match &rotation {
                Rotation::Left(_) => Rotation::Left(1),
                Rotation::Right(_) => Rotation::Right(1),
            };
            position.rotate(single_step_rotation, &mut zero_count);
        }
    }
    zero_count.0
}

fn main() {
    let path = Path::new("src/part_1_input.txt");
    let rotations = load_input(path);
    let part_1 = part_1(rotations.clone());
    println!("part_1: {}", part_1);
    let part_2 = part_2(rotations);
    println!("part_2: {}", part_2);
}

#[test]
fn test_run() {
    let rotations = vec![
        Rotation::Left(68),
        Rotation::Left(30),
        Rotation::Right(48),
        Rotation::Left(5),
        Rotation::Right(60),
        Rotation::Left(55),
        Rotation::Left(1),
        Rotation::Left(99),
        Rotation::Right(14),
        Rotation::Left(82),
    ];
    assert_eq!(part_1(rotations), 3);
}
