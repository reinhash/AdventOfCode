use std::fs::File;
use std::io::prelude::*;

fn get_calibration_value(line: &str) -> i64 {
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
        assert_eq!(get_calibration_value("11bb2".into()), 12);
        assert_eq!(get_calibration_value("bw11bb2444".into()), 14);
        assert_eq!(get_calibration_value("wawadawdwadwa".into()), 0);
    }
}
