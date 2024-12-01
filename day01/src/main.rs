use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    get_locations(
        "day01/resources/data.txt".to_string(),
        &mut list1,
        &mut list2,
    )
    .unwrap();
    let total_distance = get_distance(&list1, &list2);
    println!("{total_distance}");

    let similiarity = get_similiarity(&list1, &list2);
    println!("{similiarity}");
    Ok(())
}

fn get_locations(
    path: String,
    list1: &mut Vec<i32>,
    list2: &mut Vec<i32>,
) -> Result<(), io::Error> {
    let file = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        let numbers: Vec<i32> = line?
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if numbers.len() == 2 {
            list1.push(numbers[0]);
            list2.push(numbers[1]);
        }
    }
    list1.sort();
    list2.sort();
    Ok(())
}

fn get_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }
    sum
}

fn get_similiarity(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut similiarity = 0;
    let mut list2_occurrences_map = std::collections::HashMap::new();
    for item in list2.iter() {
        *list2_occurrences_map.entry(item).or_insert(0) += 1;
    }
    for &item in list1.iter() {
        let mut occurence = 0;
        if list2_occurrences_map.contains_key(&item) {
            occurence = *list2_occurrences_map.get(&item).unwrap();
        }
        similiarity += occurence * item;
    }
    similiarity
}
