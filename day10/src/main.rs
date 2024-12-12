use shared::read_numbers;
use std::collections::{HashMap, HashSet};

use std::io::{self};

fn main() -> io::Result<()> {
    let numbers: Vec<Vec<i32>> = read_numbers("day10/resources/data.txt")?;
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut zeros: HashSet<(usize, usize)> = HashSet::new();
    let mut nines: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    set_graph(&mut graph, &mut zeros, &mut nines, &numbers, &mut visited);
    let sum = find_nines(&graph, &nines, &zeros, &visited, 1);
    println!("{}", sum);
    let sum = find_nines(&graph, &nines, &zeros, &visited, 2);
    println!("{}", sum);
    Ok(())
}

fn set_graph(
    graph: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
    zeros: &mut HashSet<(usize, usize)>,
    nines: &mut HashSet<(usize, usize)>,
    numbers: &Vec<Vec<i32>>,
    visited: &mut HashMap<(usize, usize), bool>,
) {
    let max_x = numbers[0].len();
    let max_y = numbers.len();
    for i in 0..max_y {
        for j in 0..max_x {
            if numbers[i][j] == 0 {
                zeros.insert((i, j));
            } else if numbers[i][j] == 9 {
                nines.insert((i, j));
            }
            visited.insert((i, j), false);
            let mut neighbors: Vec<(usize, usize)> = Vec::new();
            for k in -1..2 {
                for l in -1..2 {
                    /*exclude self and diagnonals */
                    if k == l || k * l != 0 {
                        continue;
                    }
                    let y = i as i32 + l;
                    let x = j as i32 + k;

                    if x >= 0 && x < max_x as i32 && y >= 0 && y < max_y as i32 {
                        /*accept it if it's exacly one plus */
                        if numbers[y as usize][x as usize] - numbers[i][j] == 1 {
                            neighbors.push((y as usize, x as usize));
                        }
                    }
                }
            }
            graph.insert((i, j), neighbors);
        }
    }
}

fn depth_first_search(
    at: &(usize, usize),
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    nines: &mut HashSet<(usize, usize)>,
    visited: &mut HashMap<(usize, usize), bool>,
    count: &mut u32,
) {
    if nines.is_empty() {
        return;
    }

    if *visited.entry(*at).or_default() {
        return;
    }
    visited.insert(*at, true);

    if nines.contains(at) {
        nines.remove(at);

        *count += 1;
        return;
    }

    let neighbors = graph.get(at).unwrap();
    for neighbor in neighbors {
        depth_first_search(neighbor, graph, nines, visited, count);
    }
}

fn find_nines(
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    nines: &HashSet<(usize, usize)>,
    zeros: &HashSet<(usize, usize)>,
    visited: &HashMap<(usize, usize), bool>,
    part: u8,
) -> u32 {
    let mut count = 0;
    for zero in zeros {
        let mut visited = visited.clone();
        let mut nines = nines.clone();
        if part == 2 {
            depth_search(zero, graph, &nines, &mut count);
        } else {
            depth_first_search(zero, graph, &mut nines, &mut visited, &mut count);
        }
    }
    count
}

fn depth_search(
    at: &(usize, usize),
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    nines: &HashSet<(usize, usize)>,
    count: &mut u32,
) {
    if nines.contains(at) {
        *count += 1;
        return;
    }

    let neighbors = graph.get(at).unwrap();
    for neighbor in neighbors {
        depth_search(neighbor, graph, nines, count);
    }
}
