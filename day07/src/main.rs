use shared::read_lines;
use std::{io, u64};

fn main() -> io::Result<()> {
    let lines = read_lines("day07/resources/data.txt")?;
    let mut numbers = parse_numbers(lines);
    println!("{}", numbers.len());
    let total = compare_numbers(&mut numbers);
    println!("{}", numbers.len());
    println!("total part 1: {}", total);
    let total2 = concatenate_numbers(&mut numbers);
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

fn concatenate(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}

fn calculate_result2(row: &Vec<u64>) -> Vec<u64> {
    let mut total = Vec::new();
    if row.len() == 2 {
        total.push(row[1]);
        return total;
    }
    let op_count = row.len() - 2; // first one is the result

    let combinations = create_combinations(op_count);
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
        total.push(result);
    }
    total
}

fn concatenate_numbers(numbers: &mut Vec<Vec<u64>>) -> u64 {
    let mut total = 0;

    'outer: for row in numbers {
        if row.len() == 3 {
            if row[0] == concatenate(row[1], row[2]) {
                total += row[0];
            }
        } else {
            let possible_comp = row.len() - 2;
            for i in 0..possible_comp {
                let mut old_row = row.clone();
                let mut new_row0 = Vec::new();
                let mut new_row1 = Vec::new();

                for _j in 0..i + 2 {
                    new_row0.push(old_row.remove(0));
                }

                new_row1.push(new_row0[0]);
                for _i in i + 2..row.len() {
                    new_row1.push(old_row.remove(0));
                }
                let result0 = calculate_result2(&new_row0);
                let result1 = calculate_result2(&new_row1);
                for r0 in &result0 {
                    for r1 in result1.iter() {
                        let result = concatenate(*r0, *r1);
                        if result == row[0] {
                            total += result;
                            continue 'outer;
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
    }
    total
}

/*Wrong */
fn concatenate_adjacent_numbers(numbers: &mut Vec<Vec<u64>>) -> u64 {
    let mut total = 0;
    'outer: for row in numbers {
        if row.len() == 3 {
            if row[0] == concatenate(row[1], row[2]) {
                total += row[0];
            }
        } else {
            let possible_comp = row.len() - 2;
            println!("old {:?}", row);
            for i in 0..possible_comp {
                let mut old_row = row.clone();
                let mut new_row = Vec::new();
                //println!("{:?}", row);
                for _j in 0..(i + 1) {
                    new_row.push(old_row.remove(0));
                }
                let conctenated = concatenate(old_row.remove(0), old_row.remove(0));
                new_row.push(conctenated);
                new_row.append(&mut old_row);
                println!("new {:?}", new_row);
                let result = calculate_result(&new_row);
                if result > 0 {
                    total += result;
                    continue 'outer;
                } else {
                    continue;
                }
            }
        }
    }
    total
}
