use crate::Rotation;

use std::io::Result;
use std::{fs::File, io::BufRead, io::BufReader, path::Path};

pub fn load_input(path: &Path) -> Vec<Rotation> {
    let data = parse_file(path).unwrap();
    let mut rotations = vec![];
    for item in data {
        rotations.push(item.into());
    }
    rotations
}

fn parse_file(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().collect()
}
