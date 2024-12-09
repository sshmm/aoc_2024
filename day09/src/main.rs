use std::fs::File;
use std::io::{BufRead, BufReader};

use std::{
    collections::{HashMap, HashSet},
    io::{self},
};
fn main() -> io::Result<()> {
    let mut blocks0: Vec<i32> = Vec::new();
    parse_characters("day09/resources/data.txt", &mut blocks0);
    defragement(&mut blocks0);
    let mut sum: u64 = 0;
    for (idx, b) in blocks0.iter().enumerate() {
        if *b == -1 {
            break;
        }
        sum += idx as u64 * *b as u64;
    }
    //println!("{:?}", blocks0);
    println!("{}", sum);

    let mut blocks1: Vec<(i32, i32)> = Vec::new();
    parse_characters2("day09/resources/data.txt", &mut blocks1);
    println!("{:?}\n\n\n", blocks1);
    let sum = defragement2(&mut blocks1);
    println!("{:?}", blocks1);
    println!("{}", sum);
    Ok(())
}

fn parse_characters(path: &str, list: &mut Vec<i32>) {
    let reader: BufReader<File> = BufReader::new(File::open(path).expect("error reading file"));
    let characters: Vec<char> = reader
        .lines()
        .flat_map(|l| l.expect("line failed").chars().collect::<Vec<_>>())
        .collect();

    for (idx, c) in characters.iter().enumerate() {
        let mut block = c.to_string().parse::<u32>().unwrap();
        if block == 0 {
            continue;
        }
        if idx % 2 == 0 {
            while block > 0 {
                let id = idx as i32 / 2;
                list.push(id);
                block -= 1;
            }
        } else {
            while block > 0 {
                list.push(-1);
                block -= 1;
            }
        }
    }
}

fn defragement(list: &mut Vec<i32>) {
    let mut free_idx = 0;
    'outer: for block_idx in (0..list.len()).rev() {
        if list[block_idx] == -1 {
            continue;
        }

        for i in free_idx..list.len() {
            if i >= block_idx {
                break 'outer;
            }
            if list[i] == -1 {
                list[i] = list[block_idx];
                list[block_idx] = -1;
                free_idx = i + 1;
                break;
            }
        }
    }
}

fn parse_characters2(path: &str, list: &mut Vec<(i32, i32)>) {
    let reader: BufReader<File> = BufReader::new(File::open(path).expect("error reading file"));
    let characters: Vec<char> = reader
        .lines()
        .flat_map(|l| l.expect("line failed").chars().collect::<Vec<_>>())
        .collect();

    for (idx, c) in characters.iter().enumerate() {
        let block = c.to_string().parse::<i32>().unwrap();
        if block == 0 {
            continue;
        }

        if idx % 2 == 0 {
            let id = idx as i32 / 2;
            list.push((id, block));
        } else {
            list.push((-1, block));
        }
    }
}

fn defragement2(list: &mut Vec<(i32, i32)>) -> u64 {
    let mut block_idx = list.len() - 1;
    'outer: while block_idx > 0 {
        if list[block_idx].1 <= 0 || list[block_idx].0 <= 0 {
            block_idx -= 1;
            continue;
        }

        for i in 0..list.len() {
            if i >= block_idx {
                break;
            }
            if list[i].0 < 0 && list[i].1 >= list[block_idx].1 {
                let diff = list[i].1 - list[block_idx].1;
                if diff == 0 {
                    list[i].0 = list[block_idx].0;
                    list[block_idx].0 = -1;
                } else {
                    list[i] = list[block_idx];
                    list[block_idx].0 = -1;
                    list.insert(i + 1, (-1, diff));
                    block_idx += 1; //next element shifted
                }
                break;
            }
        }
        println!("{}", block_idx);
        block_idx -= 1;
    }

    let mut index = 0;
    let mut sum: u64 = 0;
    for file in list {
        for _i in 0..file.1 {
            if file.0 != -1 {
                sum += index as u64 * file.0 as u64;
            }
            index += 1;
        }
    }

    sum
}
