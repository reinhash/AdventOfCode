use std::fs::File;
use std::io::prelude::*;

fn window(line: &str) -> String {
    let mut index = 0;
    let mut line_out = String::from(line);
    loop {
        if index == line_out.len() - 1 {
            break;
        }
        let line_slice = &line_out[index..];
        if line_slice.starts_with("one") {
            line_out.replace_range(index..index + "one".len(), "1e");
            index = 0;
        } else if line_slice.starts_with("two") {
            line_out.replace_range(index..index + "two".len(), "2o");
            index = 0;
        } else if line_slice.starts_with("three") {
            line_out.replace_range(index..index + "three".len(), "3e");
            index = 0;
        } else if line_slice.starts_with("four") {
            line_out.replace_range(index..index + "four".len(), "4r");
            index = 0;
        } else if line_slice.starts_with("five") {
            line_out.replace_range(index..index + "five".len(), "5e");
            index = 0;
        } else if line_slice.starts_with("six") {
            line_out.replace_range(index..index + "six".len(), "6x");
            index = 0;
        } else if line_slice.starts_with("seven") {
            line_out.replace_range(index..index + "seven".len(), "7n");
            index = 0;
        } else if line_slice.starts_with("eight") {
            line_out.replace_range(index..index + "eight".len(), "8t");
            index = 0;
        } else if line_slice.starts_with("nine") {
            line_out.replace_range(index..index + "nine".len(), "9e");
            index = 0;
        } else {
            index += 1;
        }
    }
    line_out
}

fn get_calibration_value(line: &str) -> i64 {
    let line = window(line);
    let line: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();
    let line_length = line.len();

    match line_length {
        0 => 0,
        1 => {
            let combined_number = format!("{}{}", line[0], line[0]);
            combined_number.parse().unwrap()
        }
        _ => {
            let combined_number = format!("{}{}", line[0], line[line_length - 1]);
            combined_number.parse().unwrap()
        }
    }
}

pub fn run() {
    let mut file = File::open("src/input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");

    let total_value: i64 = contents
        .lines()
        .map(|line| get_calibration_value(line))
        .sum();
    println!("{}", total_value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calibration_value() {
        assert_eq!(get_calibration_value("two1nine".into()), 29);
        assert_eq!(get_calibration_value("eightwothree".into()), 83);
        assert_eq!(get_calibration_value("abcone2threexyz".into()), 13);
        assert_eq!(get_calibration_value("xtwone3four".into()), 24);
        assert_eq!(get_calibration_value("7pqrstsixteen".into()), 76);
        assert_eq!(get_calibration_value("zoneight234".into()), 14);
        assert_eq!(get_calibration_value("4nineeightseven2".into()), 42);
        assert_eq!(get_calibration_value("twone".into()), 21);
    }
}
