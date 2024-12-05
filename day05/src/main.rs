use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let rules = read_file("day05/resources/rules.txt", '|')?;
    let rules_map = parse_rules(&rules);
    let mut pages = read_file("day05/resources/data.txt", ',')?;
    let result = fix_pages(&mut pages, &rules_map).unwrap();
    println!("Result: {}", result);
    Ok(())
}

fn read_file(file_name: &str, separator: char) -> Result<Vec<Vec<i32>>, io::Error> {
    let file = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        numbers.push(line?.split(separator).map(|n| n.parse().unwrap()).collect());
    }

    Ok(numbers)
}

fn parse_rules(rules: &Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        rules_map.entry(rule[1]).or_insert(Vec::new()).push(rule[0]);
    }
    return rules_map;
}

fn parse_pages(pages: &Vec<Vec<i32>>, rules: &HashMap<i32, Vec<i32>>) -> Result<i32, ()> {
    let mut result = 0;
    'outer: for page in pages {
        for (key, value) in rules {
            if let Some(index) = page.iter().position(|&x| x == *key) {
                if index + 1 < page.len() && value.iter().any(|&x| page[index + 1..].contains(&x)) {
                    continue 'outer;
                }
            }
        }
        if page.len() % 2 == 1 {
            let mid = page.len() / 2;
            result += page[mid];
        } else {
            let mid = page.len() / 2;
            result += (page[mid] + page[mid - 1]) / 2;
        }
    }
    return Ok(result);
}

fn fix_pages(pages: &mut Vec<Vec<i32>>, rules: &HashMap<i32, Vec<i32>>) -> Result<i32, ()> {
    let mut result = 0;
    for page in pages {
        let mut fixed = false;
        for (key, value) in rules {
            if let Some(index) = page.iter().position(|&x| x == *key) {
                if index + 1 >= page.len() {
                    continue;
                }
                let mut current_index = index;
                for i in index + 1..page.len() {
                    if value.contains(&page[i]) {
                        println! {"Value:{:?} {} {}", value, i,page[i]};
                        let temp = page.remove(i);
                        page.insert(current_index, temp);
                        current_index += 1;
                        println!("Key:{} {:?}", key, page);
                        fixed = true;
                    }
                }
            }
        }
        if !fixed {
            continue;
        }
        if page.len() % 2 == 1 {
            let mid = page.len() / 2;
            result += page[mid];
        } else {
            let mid = page.len() / 2;
            result += (page[mid] + page[mid - 1]) / 2;
        }
    }
    return Ok(result);
}
