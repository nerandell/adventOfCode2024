use std::fs::read_to_string;
use std::io::Result;

pub fn read_file(filename: &str) -> Result<Vec<String>> {
    let mut input = vec![];

    for line in read_to_string(filename)?.lines() {
        input.push(String::from(line))
    }

    Ok(input)
}
