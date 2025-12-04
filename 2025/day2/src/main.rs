use std::path::Path;

use crate::parser::{Range, load_input};

mod parser;

#[derive(Debug)]
struct InvalidProductId(u64);

fn check_invalid_ranges_part_1(range: Range) -> Vec<InvalidProductId> {
    let mut invalid_nums = vec![];

    for num in range.first_id.0..=range.last_id.0 {
        let num_str = num.to_string();
        if num_str.len() % 2 == 0 {
            let mid = num_str.len() / 2;
            let (first_half, second_half) = num_str.split_at(mid);
            if first_half == second_half {
                invalid_nums.push(InvalidProductId(num));
            }
        }
    }
    invalid_nums
}

fn check_invalid_ranges_part_2(range: Range) -> Vec<InvalidProductId> {
    let mut invalid_nums = vec![];

    for num in range.first_id.0..=range.last_id.0 {
        let num_str = num.to_string();
        let check_range = 1..=num_str.len() / 2;
        for check in check_range {
            // Only check if the pattern length divides evenly into the total length
            if num_str.len() % check != 0 {
                continue;
            }

            let repeat_amount = num_str.len() / check;
            // Must be repeated at least twice
            if repeat_amount >= 2 {
                let check_str = num_str[0..check].repeat(repeat_amount);
                if check_str == num_str {
                    invalid_nums.push(InvalidProductId(num));
                    break;
                }
            }
        }
    }
    invalid_nums
}

fn sum_ranges(ranges: Vec<Vec<InvalidProductId>>) -> u64 {
    ranges.iter().flat_map(|v| v.iter()).map(|id| id.0).sum()
}

fn part_1(path: &Path) -> u64 {
    let ranges = load_input(path);
    let mut invalid_ranges = vec![];
    for range in ranges {
        invalid_ranges.push(check_invalid_ranges_part_1(range));
    }
    sum_ranges(invalid_ranges)
}

fn part_2(path: &Path) -> u64 {
    let ranges = load_input(path);
    let mut invalid_ranges = vec![];
    for range in ranges {
        invalid_ranges.push(check_invalid_ranges_part_2(range));
    }
    sum_ranges(invalid_ranges)
}

fn main() {
    let path = Path::new("src/part_1_input.txt");
    let result = part_1(path);
    println!("Part 1: {}", result);

    let result = part_2(path);
    println!("Part 2: {}", result);
}

#[test]
fn example() {
    let path = Path::new("src/test_input.txt");
    let result = part_1(path);
    assert_eq!(result, 1227775554);
}
