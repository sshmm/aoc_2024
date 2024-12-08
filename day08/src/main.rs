use shared::read_characters;
use std::{
    collections::{HashMap, HashSet},
    io::{self},
};
fn main() -> io::Result<()> {
    let data = read_characters("day08/resources/data.txt")?;
    let mut antnennas_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes_map: HashSet<(usize, usize)> = HashSet::new();
    find_antennas(&data, &mut antnennas_map);
    find_antinodes(
        &antnennas_map,
        &mut antinodes_map,
        data[0].len() as isize,
        data.len() as isize,
    );
    println!("{}", antinodes_map.len());
    find_antinodes2(
        &antnennas_map,
        &mut antinodes_map,
        data[0].len() as isize,
        data.len() as isize,
    );
    println!("{}", antinodes_map.len());
    Ok(())
}

fn find_antennas(data: &Vec<Vec<char>>, map: &mut HashMap<char, Vec<(usize, usize)>>) {
    for row in 0..data.len() {
        for col in 0..data[row].len() {
            let character = data[row][col];
            if character != '.' {
                map.entry(character).or_insert(Vec::new()).push((row, col));
            }
        }
    }
}

fn find_antinodes(
    map: &HashMap<char, Vec<(usize, usize)>>,
    antinodes: &mut HashSet<(usize, usize)>,
    max_x: isize,
    max_y: isize,
) {
    for locations in map.values() {
        for (idx, location) in locations.iter().enumerate() {
            for i in idx + 1..locations.len() {
                let y_diff: isize = locations[i].0 as isize - location.0 as isize;
                let x_diff: isize = locations[i].1 as isize - location.1 as isize;
                let first_antinode = (location.0 as isize - y_diff, location.1 as isize - x_diff);
                let second_antinode = (
                    locations[i].0 as isize + y_diff,
                    locations[i].1 as isize + x_diff,
                );
                if first_antinode.0 >= 0 && first_antinode.1 >= 0 && first_antinode.1 < max_x {
                    antinodes.insert((first_antinode.0 as usize, first_antinode.1 as usize));
                }
                if second_antinode.0 < max_y && second_antinode.1 >= 0 && second_antinode.1 < max_x
                {
                    antinodes.insert((second_antinode.0 as usize, second_antinode.1 as usize));
                }
            }
        }
    }
}

fn find_antinodes2(
    map: &HashMap<char, Vec<(usize, usize)>>,
    antinodes: &mut HashSet<(usize, usize)>,
    max_x: isize,
    max_y: isize,
) {
    for locations in map.values() {
        for (idx, location) in locations.iter().enumerate() {
            for i in idx + 1..locations.len() {
                antinodes.insert(*location);
                antinodes.insert(locations[i]);
                let y_diff: isize = locations[i].0 as isize - location.0 as isize;
                let x_diff: isize = locations[i].1 as isize - location.1 as isize;
                let mut first_antinode = (
                    location.0 as isize - (2 * y_diff),
                    location.1 as isize - (2 * x_diff),
                );

                let mut second_antinode = (
                    locations[i].0 as isize + (2 * y_diff),
                    locations[i].1 as isize + (2 * x_diff),
                );
                while first_antinode.0 >= 0 && first_antinode.1 >= 0 && first_antinode.1 < max_x {
                    antinodes.insert((first_antinode.0 as usize, first_antinode.1 as usize));
                    first_antinode = (first_antinode.0 - y_diff, first_antinode.1 - x_diff);
                }
                while second_antinode.0 < max_y
                    && second_antinode.1 >= 0
                    && second_antinode.1 < max_x
                {
                    antinodes.insert((second_antinode.0 as usize, second_antinode.1 as usize));
                    second_antinode = (second_antinode.0 + y_diff, second_antinode.1 + x_diff);
                }
            }
        }
    }
}
