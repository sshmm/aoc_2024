use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let numbers: Vec<u64> = read_data("day11/resources/data.txt");
    let mut sum: u64 = 0;
    let mut memo = HashMap::new();
    for number in numbers {
        split(number, 0, &mut sum, &mut memo);
    }
    println!("{}", sum);
    Ok(())
}

fn read_data(path: &str) -> Vec<u64> {
    let mut data: Vec<u64> = Vec::new();
    let file = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        for num in line.split_whitespace() {
            data.push(num.parse().unwrap());
        }
    }

    data
}

fn split(value: u64, depth: u8, count: &mut u64, memo: &mut HashMap<(u64, u8), u64>) {
    if depth == 75 {
        *count += 1;
        return;
    }

    if let Some(&cached_count) = memo.get(&(value, depth)) {
        *count += cached_count;
        return;
    }
    let initial_count = *count;
    if value == 0 {
        split(value + 1, depth + 1, count, memo);
    } else {
        let value_string = value.to_string();
        let length = value_string.len();
        if length % 2 == 1 {
            split(value * 2024, depth + 1, count, memo)
        } else {
            split(
                value_string[0..length / 2].parse().unwrap(),
                depth + 1,
                count,
                memo,
            );
            split(
                value_string[length / 2..length].parse().unwrap(),
                depth + 1,
                count,
                memo,
            );
        }
    }
    let computed_count = *count - initial_count;
    memo.insert((value, depth), computed_count);
}
