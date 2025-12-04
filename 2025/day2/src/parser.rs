use std::convert::From;

use std::io::Result;
use std::{fs::File, io::BufRead, io::BufReader, path::Path};

#[derive(Debug)]
pub struct ProductId(pub u64);

impl From<String> for ProductId {
    fn from(value: String) -> Self {
        let numeric_value: u64 = value.parse().unwrap();
        Self(numeric_value)
    }
}

#[derive(Debug)]
pub struct Range {
    pub first_id: ProductId,
    pub last_id: ProductId,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split('-').collect();
        Self {
            first_id: ProductId::from(parts[0].to_string()),
            last_id: ProductId::from(parts[1].to_string()),
        }
    }
}

pub fn load_input(path: &Path) -> Vec<Range> {
    let data = parse_file(path).unwrap();
    // There will only be one line in the input file
    data.iter()
        .next()
        .unwrap()
        .split(",")
        .map(|line| Range::from(line))
        .collect()
}

fn parse_file(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().collect()
}
