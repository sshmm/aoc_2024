use shared::read_characters;
use std::{
    collections::HashMap,
    io::{self},
};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn main() -> io::Result<()> {
    let mut data = read_characters("day06/resources/data.txt")?;
    let distinct_pos = get_steps(&mut data);
    println!("Steps: {:?}", distinct_pos);
    Ok(())
}

fn get_steps(characters: &mut Vec<Vec<char>>) -> (usize, usize) {
    let mut current_pos = (0, 0, Direction::Up);
    let mut distinct_pos = 0;
    let mut loop_possible_count = 0;
    let mut map: HashMap<(usize, usize), bool> = HashMap::new();
    'outer: for i in 0..characters.len() {
        for j in 0..characters[i].len() {
            if characters[i][j] == '^' {
                current_pos = (i, j, Direction::Up);
                characters[i][j] = 'U';
                distinct_pos += 1;
                break 'outer;
            }
        }
    }
    while current_pos.0 != 0
        && current_pos.1 != 0
        && current_pos.0 < characters.len()
        && current_pos.1 < characters[current_pos.0].len()
    {
        if (current_pos.0 == 99 && current_pos.1 == 12)
            || (current_pos.0 == 100 && current_pos.1 == 12)
            || (current_pos.0 == 100 && current_pos.1 == 11)
            || (current_pos.0 == 100 && current_pos.1 == 13)
        {
            println!("{} {} {:?}", current_pos.0, current_pos.1, current_pos.2);
        }

        match current_pos.2 {
            Direction::Up => move_up(
                characters,
                &mut current_pos,
                &mut distinct_pos,
                &mut loop_possible_count,
                &mut map,
            ),
            Direction::Right => move_right(
                characters,
                &mut current_pos,
                &mut distinct_pos,
                &mut loop_possible_count,
                &mut map,
            ),
            Direction::Down => move_down(
                characters,
                &mut current_pos,
                &mut distinct_pos,
                &mut loop_possible_count,
                &mut map,
            ),
            Direction::Left => move_left(
                characters,
                &mut current_pos,
                &mut distinct_pos,
                &mut loop_possible_count,
                &mut map,
            ),
        }
    }

    println!("{} {}", current_pos.0, current_pos.1);
    println!("{:?}", current_pos.2);
    (distinct_pos, loop_possible_count)
}

fn move_up(
    characters: &mut Vec<Vec<char>>,
    current_pos: &mut (usize, usize, Direction),
    distinct_pos: &mut usize,
    loop_possible_count: &mut usize,
    total_map: &mut HashMap<(usize, usize), bool>,
) {
    let mut current_cycle = 0;
    let mut found = false;
    if current_pos.0 == 0 {
        return;
    }
    if characters[current_pos.0 - 1][current_pos.1] == '#' {
        current_pos.2 = Direction::Right;
    } else {
        let mut temp_pos: (usize, usize, Direction) = *current_pos;
        let mut map: HashMap<(usize, usize), Vec<char>> = HashMap::new();
        map.insert((current_pos.0, current_pos.1), vec!['U']);
        let current_loops = loop_possible_count.clone();
        if characters[current_pos.0 - 1][current_pos.1] == '.' {
            *distinct_pos += 1;
            characters[current_pos.0 - 1][current_pos.1] = 'U';

            if !(total_map.contains_key(&(current_pos.0 - 1, current_pos.1))
                && *total_map.get(&(current_pos.0 - 1, current_pos.1)).unwrap())
            {
                characters[current_pos.0 - 1][current_pos.1] = '#';
                check_loop(
                    characters,
                    current_pos,
                    &mut temp_pos,
                    loop_possible_count,
                    &mut map,
                    'R',
                    &mut current_cycle,
                    &mut found,
                );
                characters[current_pos.0 - 1][current_pos.1] = 'U';
            }
        } else {
            if !(total_map.contains_key(&(current_pos.0 - 1, current_pos.1))
                && *total_map.get(&(current_pos.0 - 1, current_pos.1)).unwrap())
            {
                characters[current_pos.0 - 1][current_pos.1] = '#';
                check_loop(
                    characters,
                    current_pos,
                    &mut temp_pos,
                    loop_possible_count,
                    &mut map,
                    'R',
                    &mut current_cycle,
                    &mut found,
                );
                characters[current_pos.0 - 1][current_pos.1] = 'U';
            }
        }
        if current_loops < *loop_possible_count {
            total_map.insert((current_pos.0 - 1, current_pos.1), true);
            println!("U{} {}", current_pos.0 - 1, current_pos.1);
        }

        current_pos.0 -= 1;
    }
}

fn move_right(
    characters: &mut Vec<Vec<char>>,
    current_pos: &mut (usize, usize, Direction),
    distinct_pos: &mut usize,
    loop_possible_count: &mut usize,
    total_map: &mut HashMap<(usize, usize), bool>,
) {
    let mut current_cycle = 0;
    let mut found = false;
    if current_pos.1 + 1 == characters[current_pos.0].len() {
        return;
    }
    if characters[current_pos.0][current_pos.1 + 1] == '#' {
        current_pos.2 = Direction::Down;
    } else {
        let mut temp_pos: (usize, usize, Direction) = *current_pos;
        let mut map: HashMap<(usize, usize), Vec<char>> = HashMap::new();
        map.insert((current_pos.0, current_pos.1), vec!['R']);
        let current_loops = loop_possible_count.clone();
        if characters[current_pos.0][current_pos.1 + 1] == '.' {
            *distinct_pos += 1;
            characters[current_pos.0][current_pos.1 + 1] = 'R';
            if !(total_map.contains_key(&(current_pos.0, current_pos.1 + 1))
                && *total_map.get(&(current_pos.0, current_pos.1 + 1)).unwrap())
            {
                characters[current_pos.0][current_pos.1 + 1] = '#';
                check_loop(
                    characters,
                    current_pos,
                    &mut temp_pos,
                    loop_possible_count,
                    &mut map,
                    'D',
                    &mut current_cycle,
                    &mut found,
                );
                characters[current_pos.0][current_pos.1 + 1] = 'R';
            }
        } else {
            if !(total_map.contains_key(&(current_pos.0, current_pos.1 + 1))
                && *total_map.get(&(current_pos.0, current_pos.1 + 1)).unwrap())
            {
                characters[current_pos.0][current_pos.1 + 1] = '#';
                check_loop(
                    characters,
                    current_pos,
                    &mut temp_pos,
                    loop_possible_count,
                    &mut map,
                    'D',
                    &mut current_cycle,
                    &mut found,
                );
                characters[current_pos.0][current_pos.1 + 1] = 'R';
            }
        }
        if current_loops < *loop_possible_count {
            total_map.insert((current_pos.0, current_pos.1 + 1), true);
            println!("R{} {}", current_pos.0, current_pos.1 + 1);
        }
        current_pos.1 += 1;
    }
}

fn move_down(
    characters: &mut Vec<Vec<char>>,
    current_pos: &mut (usize, usize, Direction),
    distinct_pos: &mut usize,
    loop_possible_count: &mut usize,
    total_map: &mut HashMap<(usize, usize), bool>,
) {
    let mut found = false;
    let mut current_cycle = 0;
    if current_pos.0 + 1 == characters.len() {
        return;
    }
    if characters[current_pos.0 + 1][current_pos.1] == '#' {
        current_pos.2 = Direction::Left;
    } else {
        let mut temp_pos: (usize, usize, Direction) = *current_pos;
        let mut map: HashMap<(usize, usize), Vec<char>> = HashMap::new();
        map.insert((current_pos.0, current_pos.1), vec!['D']);
        let current_loops = loop_possible_count.clone();
        if characters[current_pos.0 + 1][current_pos.1] == '.' {
            *distinct_pos += 1;
            characters[current_pos.0 + 1][current_pos.1] = 'D';

            if !(total_map.contains_key(&(current_pos.0 + 1, current_pos.1))
                && *total_map.get(&(current_pos.0 + 1, current_pos.1)).unwrap())
            {
                characters[current_pos.0 + 1][current_pos.1] = '#';
                check_loop(
                    characters,
                    current_pos,
                    &mut temp_pos,
                    loop_possible_count,
                    &mut map,
                    'L',
                    &mut current_cycle,
                    &mut found,
                );
                characters[current_pos.0 + 1][current_pos.1] = 'D';
            }
        } else {
            if !(total_map.contains_key(&(current_pos.0 + 1, current_pos.1))
                && *total_map.get(&(current_pos.0 + 1, current_pos.1)).unwrap())
            {
                characters[current_pos.0 + 1][current_pos.1] = '#';
                check_loop(
                    characters,
                    current_pos,
                    &mut temp_pos,
                    loop_possible_count,
                    &mut map,
                    'L',
                    &mut current_cycle,
                    &mut found,
                );
            }
            characters[current_pos.0 + 1][current_pos.1] = 'D';
        }
        if current_loops < *loop_possible_count {
            total_map.insert((current_pos.0 + 1, current_pos.1), true);
            println!("D{} {}", current_pos.0 + 1, current_pos.1);
        }
        current_pos.0 += 1;
    }
}

fn move_left(
    characters: &mut Vec<Vec<char>>,
    current_pos: &mut (usize, usize, Direction),
    distinct_pos: &mut usize,
    loop_possible_count: &mut usize,
    total_map: &mut HashMap<(usize, usize), bool>,
) {
    let mut current_cycle = 0;
    let mut found: bool = false;
    if current_pos.1 == 0 {
        return;
    }
    if characters[current_pos.0][current_pos.1 - 1] == '#' {
        current_pos.2 = Direction::Up;
    } else {
        let mut map: HashMap<(usize, usize), Vec<char>> = HashMap::new();
        let mut temp_pos: (usize, usize, Direction) = *current_pos;
        map.insert((current_pos.0, current_pos.1), vec!['L']);
        let current_loops = loop_possible_count.clone();
        if characters[current_pos.0][current_pos.1 - 1] == '.' {
            *distinct_pos += 1;
            characters[current_pos.0][current_pos.1 - 1] = 'L';
            if !(total_map.contains_key(&(current_pos.0, current_pos.1 - 1))
                && *total_map.get(&(current_pos.0, current_pos.1 - 1)).unwrap())
            {
                characters[current_pos.0][current_pos.1 - 1] = '#';
                check_loop(
                    characters,
                    current_pos,
                    &mut temp_pos,
                    loop_possible_count,
                    &mut map,
                    'U',
                    &mut current_cycle,
                    &mut found,
                );
                characters[current_pos.0][current_pos.1 - 1] = 'L';
            }
        } else {
            if !(total_map.contains_key(&(current_pos.0, current_pos.1 - 1))
                && *total_map.get(&(current_pos.0, current_pos.1 - 1)).unwrap())
            {
                characters[current_pos.0][current_pos.1 - 1] = '#';
                check_loop(
                    characters,
                    current_pos,
                    &mut temp_pos,
                    loop_possible_count,
                    &mut map,
                    'U',
                    &mut current_cycle,
                    &mut found,
                );
                characters[current_pos.0][current_pos.1 - 1] = 'L';
            }
        }
        if current_loops < *loop_possible_count {
            total_map.insert((current_pos.0, current_pos.1 - 1), true);
            println!("L{} {}", current_pos.0, current_pos.1 - 1);
        }
        current_pos.1 -= 1;
    }
}

fn check_loop(
    characters: &Vec<Vec<char>>,
    current_pos: &(usize, usize, Direction),
    temp_pos: &mut (usize, usize, Direction),
    loop_possible_count: &mut usize,
    map: &mut HashMap<(usize, usize), Vec<char>>,
    direction: char,
    current_cycle: &mut usize,
    found: &mut bool,
) {
    /* * println!(
        "temp {} {} current {} {}",
        temp_pos.0, temp_pos.1, current_pos.0, current_pos.1
    )*/
    let mut x: isize = match direction {
        'R' => temp_pos.1 as isize + 1,
        'D' => temp_pos.0 as isize + 1,
        'L' => temp_pos.1 as isize - 1,
        'U' => temp_pos.0 as isize - 1,
        _ => return,
    };

    *current_cycle += 1;

    while *found == false
        && match direction {
            'R' => x < characters[temp_pos.0].len() as isize,
            'D' => x < characters.len() as isize,
            'L' => x >= 0,
            'U' => x >= 0,
            _ => false,
        }
    {
        if match direction {
            'R' => characters[temp_pos.0][x as usize] == '#',
            'D' => characters[x as usize][temp_pos.1] == '#',
            'L' => characters[temp_pos.0][x as usize] == '#',
            'U' => characters[x as usize][temp_pos.1] == '#',
            _ => false,
        } {
            let mut new_pos = match direction {
                'R' => (temp_pos.0, x as usize - 1, Direction::Down),
                'D' => (x as usize - 1, temp_pos.1, Direction::Left),
                'L' => (temp_pos.0, x as usize + 1, Direction::Up),
                'U' => (x as usize + 1, temp_pos.1, Direction::Right),
                _ => return,
            };
            check_loop(
                characters,
                current_pos,
                &mut new_pos,
                loop_possible_count,
                map,
                match direction {
                    'R' => 'D',
                    'D' => 'L',
                    'L' => 'U',
                    'U' => 'R',
                    _ => return,
                },
                current_cycle,
                found,
            );
            *found = true;
            return;
        }

        if match direction {
            'R' => {
                map.contains_key(&(temp_pos.0, x as usize))
                    && map
                        .get(&(temp_pos.0, x as usize))
                        .unwrap()
                        .contains(&direction)
            }
            'D' => {
                map.contains_key(&(x as usize, temp_pos.1))
                    && map
                        .get(&(x as usize, temp_pos.1))
                        .unwrap()
                        .contains(&direction)
            }
            'L' => {
                map.contains_key(&(temp_pos.0, x as usize))
                    && map
                        .get(&(temp_pos.0, x as usize))
                        .unwrap()
                        .contains(&direction)
            }
            'U' => {
                map.contains_key(&(x as usize, temp_pos.1))
                    && map
                        .get(&(x as usize, temp_pos.1))
                        .unwrap()
                        .contains(&direction)
            }
            _ => false,
        } {
            println!(" found {:?} {:?}", current_pos, temp_pos);
            *loop_possible_count += 1;
            *found = true;
            break;
        }

        match direction {
            'R' => map
                .entry((temp_pos.0, x as usize))
                .or_insert(Vec::new())
                .push(direction),
            'D' => map
                .entry((x as usize, temp_pos.1))
                .or_insert(Vec::new())
                .push(direction),
            'L' => map
                .entry((temp_pos.0, x as usize))
                .or_insert(Vec::new())
                .push(direction),
            'U' => map
                .entry((x as usize, temp_pos.1))
                .or_insert(Vec::new())
                .push(direction),
            _ => return,
        }

        x = match direction {
            'R' => x + 1,
            'D' => x + 1,
            'L' => x - 1,
            'U' => x - 1,
            _ => return,
        };
    }
}
