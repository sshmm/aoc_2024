use shared::read_characters;
use std::io::{self};
fn main() -> io::Result<()> {
    let data = read_characters("day04/resources/data.txt")?;
    let xmas_count = get_xmas(&data);
    let xmas_count2 = get_xmas2(&data);
    let x_mas_count = get_x_mas(&data);
    println!("XMAS count: {}", xmas_count);
    println!("XMAS count: {}", xmas_count2);
    println!("X-MAS count: {}", x_mas_count);

    Ok(())
}

fn get_xmas(characters: &Vec<Vec<char>>) -> i32 {
    /*all rows have same length */
    let row_len = characters[0].len() as isize;
    let column_len = characters.len() as isize;
    let mut result = 0;
    for i in 0..column_len {
        for j in 0..row_len {
            if characters[i as usize][j as usize] == 'X' {
                if j + 1 < row_len - 2
                    && characters[i as usize][(j + 1) as usize] == 'M'
                    && characters[i as usize][(j + 2) as usize] == 'A'
                    && characters[i as usize][(j + 3) as usize] == 'S'
                {
                    result += 1;
                }

                if j - 1 >= 2
                    && characters[i as usize][(j - 1) as usize] == 'M'
                    && characters[i as usize][(j - 2) as usize] == 'A'
                    && characters[i as usize][(j - 3) as usize] == 'S'
                {
                    result += 1;
                }

                if i + 1 < column_len - 2
                    && characters[(i + 1) as usize][j as usize] == 'M'
                    && characters[(i + 2) as usize][j as usize] == 'A'
                    && characters[(i + 3) as usize][j as usize] == 'S'
                {
                    result += 1;
                }
                if i - 1 >= 2
                    && characters[(i - 1) as usize][j as usize] == 'M'
                    && characters[(i - 2) as usize][j as usize] == 'A'
                    && characters[(i - 3) as usize][j as usize] == 'S'
                {
                    result += 1;
                }

                if i + 1 < column_len - 2
                    && j + 1 < row_len - 2
                    && characters[(i + 1) as usize][(j + 1) as usize] == 'M'
                    && characters[(i + 2) as usize][(j + 2) as usize] == 'A'
                    && characters[(i + 3) as usize][(j + 3) as usize] == 'S'
                {
                    result += 1;
                }

                if i - 1 >= 2
                    && j - 1 >= 2
                    && characters[(i - 1) as usize][(j - 1) as usize] == 'M'
                    && characters[(i - 2) as usize][(j - 2) as usize] == 'A'
                    && characters[(i - 3) as usize][(j - 3) as usize] == 'S'
                {
                    result += 1;
                }
                if i + 1 < column_len - 2
                    && j - 1 >= 2
                    && characters[(i + 1) as usize][(j - 1) as usize] == 'M'
                    && characters[(i + 2) as usize][(j - 2) as usize] == 'A'
                    && characters[(i + 3) as usize][(j - 3) as usize] == 'S'
                {
                    result += 1;
                }

                if i - 1 >= 2
                    && j + 1 < row_len - 2
                    && characters[(i - 1) as usize][(j + 1) as usize] == 'M'
                    && characters[(i - 2) as usize][(j + 2) as usize] == 'A'
                    && characters[(i - 3) as usize][(j + 3) as usize] == 'S'
                {
                    result += 1;
                }
            }
        }
    }
    result
}

/*more universal sol */
fn get_xmas2(characters: &Vec<Vec<char>>) -> i32 {
    /*all rows have same length */
    let word = "XMAS";
    let row_len = characters[0].len() as isize;
    let column_len = characters.len() as isize;
    let mut result = 0;
    for i in 0..column_len {
        for j in 0..row_len {
            if characters[i as usize][j as usize] == 'X' {
                if j + 3 < row_len
                    && characters[i as usize][j as usize..j as usize + 4]
                        .iter()
                        .collect::<String>()
                        == word
                {
                    result += 1;
                }
                if j - 3 >= 0
                    && (0..word.len())
                        .map(|k| characters[i as usize][j as usize - k])
                        .collect::<String>()
                        == word
                {
                    result += 1;
                }
                if i + 3 < column_len
                    && (0..word.len())
                        .map(|k| characters[i as usize + k][j as usize])
                        .collect::<String>()
                        == word
                {
                    result += 1;
                }
                if i - 3 >= 0
                    && (0..word.len())
                        .map(|k| characters[i as usize - k][j as usize])
                        .collect::<String>()
                        == word
                {
                    result += 1;
                }
                if i + 3 < column_len && j + 3 < row_len {
                    let diag_word: String = (0..word.len())
                        .map(|k| characters[i as usize + k][j as usize + k])
                        .collect();
                    if diag_word == word {
                        result += 1;
                    }
                }

                if i - 3 >= 0 && j - 3 >= 0 {
                    let diag_word: String = (0..word.len())
                        .map(|k| characters[i as usize - k][j as usize - k])
                        .collect();
                    if diag_word == word {
                        result += 1;
                    }
                }

                if i - 3 >= 0 && j + 3 < row_len {
                    let diag_word: String = (0..word.len())
                        .map(|k| characters[i as usize - k][j as usize + k])
                        .collect();
                    if diag_word == word {
                        result += 1;
                    }
                }

                if i + 3 < column_len && j - 3 >= 0 {
                    let diag_word: String = (0..word.len())
                        .map(|k| characters[i as usize + k][j as usize - k])
                        .collect();
                    if diag_word == word {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn get_x_mas(characters: &Vec<Vec<char>>) -> i32 {
    /*all rows have same length */
    let row_len = characters[0].len() as isize;
    let column_len = characters.len() as isize;
    let mut result = 0;
    for i in 1..column_len - 1 {
        for j in 1..row_len - 1 {
            if characters[i as usize][j as usize] == 'A' {
                if characters[(i - 1) as usize][(j - 1) as usize] == 'M'
                    && characters[(i - 1) as usize][(j + 1) as usize] == 'S'
                    && characters[(i + 1) as usize][(j + 1) as usize] == 'S'
                    && characters[(i + 1) as usize][(j - 1) as usize] == 'M'
                {
                    result += 1;
                } else if characters[(i - 1) as usize][(j - 1) as usize] == 'M'
                    && characters[(i - 1) as usize][(j + 1) as usize] == 'M'
                    && characters[(i + 1) as usize][(j + 1) as usize] == 'S'
                    && characters[(i + 1) as usize][(j - 1) as usize] == 'S'
                {
                    result += 1;
                } else if characters[(i - 1) as usize][(j - 1) as usize] == 'S'
                    && characters[(i - 1) as usize][(j + 1) as usize] == 'M'
                    && characters[(i + 1) as usize][(j + 1) as usize] == 'M'
                    && characters[(i + 1) as usize][(j - 1) as usize] == 'S'
                {
                    result += 1;
                } else if characters[(i - 1) as usize][(j - 1) as usize] == 'S'
                    && characters[(i - 1) as usize][(j + 1) as usize] == 'S'
                    && characters[(i + 1) as usize][(j + 1) as usize] == 'M'
                    && characters[(i + 1) as usize][(j - 1) as usize] == 'M'
                {
                    result += 1;
                }
            }
        }
    }
    result
}
