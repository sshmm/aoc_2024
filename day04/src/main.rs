use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let data = read_file("day04/resources/data.txt")?;
    let xmas_count = get_xmas(&data.clone());
    let x_mas_count = get_x_mas(&data);
    println!("XMAS count: {}", xmas_count);
    println!("X-MAS count: {}", x_mas_count);

    Ok(())
}

fn read_file(file_name: &str) -> Result<Vec<Vec<char>>, io::Error> {
    let file = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut characters: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        characters.push(line?.chars().collect());
    }

    Ok(characters)
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
