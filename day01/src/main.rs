use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("day01/resources/data.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
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
    let mut sum = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }
    println!("{sum}");

    let mut similiarity = 0;
    for item in list1.iter() {
        let mut occurence = 0;
        let mut found = false;
        'inner: for item2 in list2.iter() {
            if item == item2 {
                occurence += 1;
                found = true;
            } else {
                if found {
                    //the list is ordered so no need to continue
                    break 'inner;
                }
            }
        }
        similiarity += occurence * item;
    }
    println!("{similiarity}");
    Ok(())
}
