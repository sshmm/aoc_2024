use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let data = read_file("day03/resources/data.txt")?;
    let pattern = r"mul\(\d{1,3},\d{1,3}\)";
    let mut result = 0;

    result += process_pattern(&data, pattern);
    println!("part 1 results: {}", result);

    let do_pattern = r"do\(\)";
    let dont_pattern = r"don\'t\(\)";
    let first_dont = find_pattern(&data, dont_pattern).unwrap_or(data.len());
    println!("First don't: {}", first_dont);

    result = 0;
    result += process_pattern(&data[..first_dont], pattern);
    println!("part 2-1 results: {}", result);

    let mut input = data[first_dont + 1..].to_string();
    result += process_do_patterns(&mut input, pattern, do_pattern, dont_pattern);
    println!("results: {}", result);

    Ok(())
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find_pattern(input: &str, pattern: &str) -> Option<usize> {
    let re = Regex::new(pattern).unwrap();
    re.find(input).map(|mat| mat.start())
}

fn process_pattern(input: &str, pattern: &str) -> i32 {
    let mut result = 0;
    let mut input = input.to_string();

    while let Some(start) = find_pattern(&input, pattern) {
        let end = find_pattern(&input[start..], r"\)").unwrap_or(0);
        if let Some(slice) = input.get(start + 4..start + end) {
            let numbers = slice.split(',').collect::<Vec<&str>>();
            result += numbers[0].parse::<i32>().unwrap() * numbers[1].parse::<i32>().unwrap();
            input = input[start + end..].to_string();
        } else {
            break;
        }
    }

    result
}

fn process_do_patterns(
    input: &mut String,
    pattern: &str,
    do_pattern: &str,
    dont_pattern: &str,
) -> i32 {
    let mut result = 0;

    while !input.is_empty() {
        let do_mul = find_pattern(&input, do_pattern).unwrap_or(input.len());
        if do_mul == input.len() {
            break;
        }
        *input = input[do_mul + 4..].to_string();

        while let Some(start) = find_pattern(&input, pattern) {
            let dont = find_pattern(&input, dont_pattern).unwrap_or(input.len());
            if dont < start {
                *input = input[dont + 1..].to_string();
                break;
            }
            let end = find_pattern(&input[start..], r"\)").unwrap_or(0);
            if let Some(slice) = input.get(start + 4..start + end) {
                let numbers = slice.split(',').collect::<Vec<&str>>();
                if numbers.len() == 2 {
                    result +=
                        numbers[0].parse::<i32>().unwrap() * numbers[1].parse::<i32>().unwrap();
                }
                *input = input[start + end..].to_string();
            } else {
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pattern() {
        let string = "xxxmul(12,13)ttt";
        assert_eq!(find_pattern(string, r"mul\(\d{1,3},\d{1,3}\)"), Some(3));
        assert_eq!(find_pattern(&string[3..], r"\)"), Some(9));
        assert_eq!(&string[3..13], "mul(12,13)");
    }
}
