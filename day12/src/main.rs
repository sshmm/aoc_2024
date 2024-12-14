use shared::read_characters;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let chars: Vec<Vec<char>> = read_characters("day12/resources/data.txt")?;
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    set_graph(&mut graph, &chars, &mut visited);
    let cost = count_area_cost(&graph, &chars, &mut visited);
    println!("Cost = {}", cost);
    Ok(())
}

fn count_area_cost(
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    chars: &Vec<Vec<char>>,
    visited: &mut HashMap<(usize, usize), bool>,
) -> u32 {
    let mut total_cost = 0;
    for row in 0..chars.len() {
        for col in 0..chars[0].len() {
            let mut sides = 0;
            let mut area = 0;
            depth_first_search_2(&(row, col), graph, visited, &mut sides, &mut area, chars);
            if area > 0 {
                total_cost += sides * area;
            }
        }
    }
    total_cost
}

fn depth_first_search(
    at: &(usize, usize),
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    visited: &mut HashMap<(usize, usize), bool>,
    sides: &mut u32,
    area: &mut u32,
    chars: &Vec<Vec<char>>,
) {
    if *visited.get(at).unwrap() {
        return;
    }
    *visited.entry(*at).or_insert(false) = true;

    let max_col = chars[0].len() as i32;
    let max_row = chars.len() as i32;

    for i in -1..2 {
        for j in -1..2 {
            /*exclude self and diagnonals */
            if i == j || i * j != 0 {
                continue;
            }
            let n_row = at.0 as i32 + i;
            let n_col = at.1 as i32 + j;

            if n_row == -1 || n_row == max_row || n_col == -1 || n_col == max_col {
                *sides += 1;
            } else if chars[n_row as usize][n_col as usize] != chars[at.0][at.1] {
                *sides += 1;
            }
        }
    }

    *area += 1;

    let neighbors = graph.get(at).unwrap();
    for neighbor in neighbors {
        depth_first_search(&neighbor, graph, visited, sides, area, chars);
    }
}

fn depth_first_search_2(
    at: &(usize, usize),
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    visited: &mut HashMap<(usize, usize), bool>,
    sides: &mut u32,
    area: &mut u32,
    chars: &Vec<Vec<char>>,
) {
    if *visited.get(at).unwrap() {
        return;
    }
    *visited.entry(*at).or_insert(false) = true;
    let neighbors = graph.get(at).unwrap();

    let max_col = chars[0].len();
    let max_row = chars.len();

    println!("{} at = {:?}", chars[at.0][at.1], at);
    for i in -1..2 {
        for j in -1..2 {
            /*exclude self and diagnonals */
            if i == j || i * j != 0 {
                continue;
            }
            let n_row = at.0 as i32 + i;
            let n_col = at.1 as i32 + j;
            let mut sub = false;
            if j == 0 {
                if n_row == max_row as i32 || n_row == -1 {
                    *sides += 1;
                    if at.1 < max_col - 1 {
                        for col in at.1 + 1..max_col {
                            let neighbor = (at.0 as usize, col as usize);
                            if chars[neighbor.0][neighbor.1] != chars[at.0][at.1] {
                                break;
                            }
                            if *visited.get(&neighbor).unwrap() {
                                *sides -= 1;
                                sub = true;
                                println!("{:?}", neighbor);

                                break;
                            }
                        }
                    }
                    if at.1 > 0 && !sub {
                        for col in (0..at.1).rev() {
                            let neighbor = (at.0 as usize, col as usize);
                            if chars[at.0][col as usize] != chars[at.0][at.1] {
                                break;
                            }
                            if *visited.get(&neighbor).unwrap() {
                                println!("{:?}", neighbor);
                                *sides -= 1;
                                break;
                            }
                        }
                    }
                } else if chars[n_row as usize][n_col as usize] != chars[at.0][at.1] {
                    *sides += 1;
                    if at.1 < max_col - 1 {
                        for col in at.1 + 1..max_col {
                            let neighbor = (at.0 as usize, col as usize);
                            if chars[neighbor.0][neighbor.1 as usize] != chars[at.0][at.1]
                                || chars[n_row as usize][neighbor.1]
                                    == chars[neighbor.0][neighbor.1]
                            {
                                break;
                            }

                            if *visited.get(&neighbor).unwrap() {
                                println!("{:?}", neighbor);
                                *sides -= 1;
                                sub = true;
                                break;
                            }
                        }
                    }
                    if at.1 > 0 && !sub {
                        for col in (0..at.1).rev() {
                            let neighbor = (at.0 as usize, col as usize);
                            if chars[neighbor.0][neighbor.1] != chars[at.0][at.1]
                                || chars[n_row as usize][neighbor.1]
                                    == chars[neighbor.0][neighbor.1]
                            {
                                break;
                            }

                            if *visited.get(&neighbor).unwrap() {
                                println!("{:?}", neighbor);

                                *sides -= 1;
                                break;
                            }
                        }
                    }
                }
            }
            if i == 0 {
                if n_col == max_col as i32 || n_col == -1 {
                    *sides += 1;
                    if at.0 < max_row - 1 {
                        for row in at.0 + 1..max_row {
                            let neighbor = (row as usize, at.1);
                            if chars[neighbor.0][neighbor.1] != chars[at.0][at.1] {
                                break;
                            }
                            if *visited.get(&neighbor).unwrap() {
                                println!("x{:?}", neighbor);

                                sub = true;
                                *sides -= 1;
                                break;
                            }
                        }
                    }
                    if at.0 > 0 && !sub {
                        for row in (0..at.0).rev() {
                            let neighbor = (row as usize, at.1);
                            if chars[neighbor.0][neighbor.1] != chars[at.0][at.1] {
                                break;
                            }
                            if *visited.get(&neighbor).unwrap() {
                                println!("{:?}", neighbor);

                                *sides -= 1;
                                break;
                            }
                        }
                    }
                } else if chars[n_row as usize][n_col as usize] != chars[at.0][at.1] {
                    *sides += 1;
                    if at.0 < max_row - 1 {
                        for row in at.0 + 1..max_row {
                            let neighbor = (row as usize, at.1);
                            if chars[neighbor.0][neighbor.1] != chars[at.0][at.1]
                                || chars[neighbor.0][n_col as usize]
                                    == chars[neighbor.0][neighbor.1]
                            {
                                break;
                            }
                            if *visited.get(&neighbor).unwrap() {
                                *sides -= 1;
                                println!("n{:?}", neighbor);

                                sub = true;
                                break;
                            }
                        }
                    }
                    if at.0 > 0 && !sub {
                        for row in (0..at.0).rev() {
                            let neighbor = (row as usize, at.1);
                            if chars[neighbor.0][neighbor.1] != chars[at.0][at.1]
                                || chars[neighbor.0][n_col as usize]
                                    == chars[neighbor.0][neighbor.1]
                            {
                                println!("nn");
                                break;
                            }
                            if *visited.get(&neighbor).unwrap() {
                                *sides -= 1;
                                println!("{:?}", neighbor);

                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    *area += 1;
    println!("sides = {}, area = {}", sides, area);
    for neighbor in neighbors {
        depth_first_search_2(&neighbor, graph, visited, sides, area, chars);
    }
}

fn set_graph(
    graph: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
    chars: &Vec<Vec<char>>,
    visited: &mut HashMap<(usize, usize), bool>,
) {
    let max_col = chars[0].len();
    let max_row = chars.len();
    for row in 0..max_row {
        for col in 0..max_col {
            visited.insert((row, col), false);
            let mut neighbors: Vec<(usize, usize)> = Vec::new();
            for i in -1..2 {
                for j in -1..2 {
                    /*exclude self and diagnonals */
                    if i == j || i * j != 0 {
                        continue;
                    }
                    let n_row = row as i32 + i;
                    let n_col = col as i32 + j;

                    if n_row >= 0 && n_row < max_row as i32 && n_col >= 0 && n_col < max_col as i32
                    {
                        /*accept it if it's exacly one plus */
                        if chars[n_row as usize][n_col as usize] == chars[row][col] {
                            neighbors.push((n_row as usize, n_col as usize));
                        }
                    }
                }
            }
            graph.insert((row, col), neighbors);
        }
    }
}
