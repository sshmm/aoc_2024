use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_characters(file_name: &str) -> Result<Vec<Vec<char>>, io::Error> {
    let file = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut characters: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        characters.push(line?.chars().collect());
    }

    Ok(characters)
}

pub fn read_lines(file_name: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}
