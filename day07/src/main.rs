use shared::read_lines;
use std::{io, u64};

fn main() -> io::Result<()> {
    let lines = read_lines("day07/resources/data.txt")?;
    let mut numbers = parse_numbers(lines);
    println!("{}", numbers.len());
    let total = compare_numbers(&mut numbers);
    println!("{}", numbers.len());
    println!("total part 1: {}", total);
    let total2 = compare_numbers2(&mut numbers);
    println!("{}", total);
    println!("{}", total2);
    println!("{}", total2 + total);
    Ok(())
}

fn parse_numbers(lines: Vec<String>) -> Vec<Vec<u64>> {
    let mut numbers = Vec::new();
    for line in lines {
        let row = line
            .split(|c: char| c == ':' || c == ' ')
            .filter(|s| !s.is_empty()) // Remove empty parts
            .map(|s| s.parse::<u64>().expect("Failed to parse a number"))
            .collect();
        numbers.push(row);
    }
    numbers
}

fn create_combinations(op_count: usize) -> Vec<Vec<char>> {
    let mut combinations: Vec<Vec<char>> = vec![vec![' '; op_count]; 1 << op_count];
    for col in 0..op_count {
        let mut count = (1 << (col + 1)) / 2;
        let mut switch = false;
        for row in 0..(1 << op_count) {
            if count == 0 {
                switch = !switch;
                count = (1 << (col + 1)) / 2;
            }
            if switch {
                combinations[row][col] = '+';
            } else {
                combinations[row][col] = '*';
            }
            count -= 1;
        }
    }
    combinations
}

fn create_combinations2(op_count: usize) -> Vec<Vec<char>> {
    let base: usize = 3;
    let mut combinations: Vec<Vec<char>> = vec![vec![' '; op_count]; base.pow(op_count as u32)];
    for col in 0..op_count {
        let count = base.pow(col as u32);
        for row in 0..base.pow(op_count as u32) {
            let op = match (row / count) % base {
                0 => '+',
                1 => '*',
                2 => 'C',
                _ => panic!("Invalid operator"),
            };
            combinations[row][col] = op;
        }
    }
    println!("Here");
    combinations
        .into_iter()
        .filter(|c| c.contains(&'C'))
        .collect() // Filter out the ones with C
}

fn calculate_result(row: &Vec<u64>) -> u64 {
    let op_count = row.len() - 2; // first one is the result
    let result0 = row[0];
    let combinations: Vec<Vec<char>> = create_combinations(op_count);
    'outer: for combination in combinations {
        //  println!("Trying combination: {:?}", combination);
        let mut result: u64 = row[1];
        for (i, op) in combination.iter().enumerate() {
            let rhs = row[i + 2];
            result = match op {
                '+' => match result.checked_add(rhs) {
                    Some(value) => value,
                    None => {
                        println!("Addition overflow, skipping iteration.");
                        continue 'outer; // Skip this iteration if addition overflows
                    }
                },
                '*' => match result.checked_mul(rhs) {
                    Some(value) => value,
                    None => {
                        println!("Multiplication overflow, skipping iteration.");
                        continue 'outer; // Skip this iteration if multiplication overflows
                    }
                },
                _ => panic!("Invalid operator"),
            };
        }
        if result0 == result {
            //    println!("Found a match: {:?}", combination);
            return result;
        }
    }
    return 0;
}

fn calculate_result2(row: &Vec<u64>) -> u64 {
    let op_count = row.len() - 2; // first one is the result
    let result0 = row[0];
    let combinations: Vec<Vec<char>> = create_combinations2(op_count);
    'outer: for combination in combinations {
        //  println!("Trying combination: {:?}", combination);
        let mut result: u64 = row[1];
        for (i, op) in combination.iter().enumerate() {
            let rhs = row[i + 2];
            result = match op {
                '+' => match result.checked_add(rhs) {
                    Some(value) => value,
                    None => {
                        println!("Addition overflow, skipping iteration.");
                        continue 'outer; // Skip this iteration if addition overflows
                    }
                },
                '*' => match result.checked_mul(rhs) {
                    Some(value) => value,
                    None => {
                        println!("Multiplication overflow, skipping iteration.");
                        continue 'outer; // Skip this iteration if multiplication overflows
                    }
                },
                'C' => concatenate(result, rhs),
                _ => panic!("Invalid operator"),
            };
        }
        if result0 == result {
            //    println!("Found a match: {:?}", combination);
            return result;
        }
    }
    return 0;
}

fn compare_numbers(numbers: &mut Vec<Vec<u64>>) -> u64 {
    let mut total = 0;
    let mut indices_to_remove = Vec::new();
    for (idx, row) in numbers.iter().enumerate() {
        let result = calculate_result(row);
        if result > 0 {
            total += result;
            indices_to_remove.push(idx);
        }
    }
    println!("{}", indices_to_remove.len());
    for &idx in indices_to_remove.iter().rev() {
        numbers.remove(idx);
    }
    total
}

fn compare_numbers2(numbers: &mut Vec<Vec<u64>>) -> u64 {
    let mut total = 0;
    let mut indices_to_remove = Vec::new();
    for (idx, row) in numbers.iter().enumerate() {
        let result = calculate_result2(row);
        if result > 0 {
            total += result;
            indices_to_remove.push(idx);
        }
    }
    println!("{}", indices_to_remove.len());
    for &idx in indices_to_remove.iter().rev() {
        numbers.remove(idx);
    }
    total
}

fn concatenate(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}
