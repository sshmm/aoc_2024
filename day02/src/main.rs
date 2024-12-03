use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reports = read_file("day02/resources/data.txt".to_string());
    let reports = match reports {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let safe_results1 = calc_safe_results(reports.clone());
    let safe_results2 = calc_tolerated_safe_result(reports);
    match safe_results1 {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    match safe_results2 {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read_file(path: String) -> Result<Vec<Vec<i32>>, std::io::Error> {
    let file = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut reports = Vec::new();
    for line in reader.lines() {
        let numbers: Vec<i32> = line?
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        reports.push(numbers);
    }
    Ok(reports)
}

fn is_safe(report: &[i32]) -> bool {
    let mut trend = Trend::Constant;
    let mut safe_levels = 0;
    for i in 0..(report.len() - 1) {
        if report[i + 1] > report[i] && report[i + 1] - report[i] <= 3 {
            if trend == Trend::Increasing || trend == Trend::Constant {
                trend = Trend::Increasing;
            } else {
                break;
            }
            safe_levels += 1;
        } else if report[i + 1] < report[i] && report[i] - report[i + 1] <= 3 {
            if trend == Trend::Decreasing || trend == Trend::Constant {
                trend = Trend::Decreasing;
            } else {
                break;
            }
            safe_levels += 1;
        } else {
            break;
        }
        if safe_levels == report.len() - 1 {
            return true;
        }
    }
    false
}

fn calc_safe_results(reports: Vec<Vec<i32>>) -> Result<i32, std::io::Error> {
    let mut sum = 0;
    for report in reports {
        if is_safe(&report) {
            sum += 1;
        }
    }
    Ok(sum)
}

fn find_faulty_level(report: &[i32]) -> Option<usize> {
    for i in 1..(report.len() - 1) {
        if i as i32 - 1 >= 0 {
            if report[i - 1] < report[i] && report[i] < report[i + 1] {
                if report[i + 1] - report[i] <= 3 && report[i] - report[i - 1] <= 3 {
                    continue;
                } else {
                    if report[i] - report[i - 1] > 3 && report[i + 1] - report[i] <= 3 {
                        return Some(i - 1);
                    } else if report[i] - report[i - 1] <= 3 && report[i + 1] - report[i] >= 3 {
                        if i + 2 > report.len() - 1
                            || (report[i + 2] > report[i] && report[i + 2] - report[i] <= 3)
                        {
                            return Some(i + 1);
                        } else {
                            return None;
                        }
                    }
                }
            } else if report[i - 1] > report[i] && report[i] > report[i + 1] {
                if report[i - 1] - report[i] <= 3 && report[i] - report[i + 1] <= 3 {
                    continue;
                } else {
                    if report[i - 1] - report[i] > 3 && report[i] - report[i + 1] <= 3 {
                        return Some(i - 1);
                    } else if report[i - 1] - report[i] <= 3 && report[i] - report[i + 1] >= 3 {
                        if i + 2 > report.len() - 1
                            || (report[i] > report[i + 2] && report[i] - report[i + 2] <= 3)
                        {
                            return Some(i + 1);
                        } else {
                            return None;
                        }
                    }
                }
            } else {
                if report[i + 1] == report[i] || report[i - 1] == report[i] {
                    return Some(i);
                } else if report[i + 1] > report[i] && report[i - 1] > report[i] {
                    return Some(i);
                } else if report[i + 1] < report[i] && report[i - 1] < report[i] {
                    if i + 2 > report.len() - 1
                        || (report[i] < report[i + 2] && report[i + 2] - report[i] <= 3)
                    {
                        return Some(i + 1);
                    } else {
                        return None;
                    }
                } else if report[i + 1] > report[i] && report[i - 1] > report[i] {
                    if i + 2 > report.len() - 1
                        || (report[i - 2] < report[i] && report[i - 2] - report[i] <= 3)
                    {
                        return Some(i + 1);
                    } else {
                        return None;
                    }
                }
            }
        } else {
            if i as i32 - 1 < 0 {
                //equality will be handled next iteration
                if (report[i + 1] - report[i]).abs() <= 3 {
                    continue;
                } else if (report[i + 1] - report[i + 2]).abs() <= 3 {
                    return Some(i);
                }
            }
        }
    }
    None
}

fn calc_tolerated_safe_result(reports: Vec<Vec<i32>>) -> Result<i32, std::io::Error> {
    let mut sum = 0;

    for report in reports {
        if report.len() == 2 {
            sum += 1;
            continue;
        }
        if is_safe(&report) {
            sum += 1;
            continue;
        }

        /*
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if is_safe(&new_report) {
                    sum += 1;
                    break;
                }
            }
        }*/
        let faulty_report = find_faulty_level(&report);
        if faulty_report.is_some() {
            println!("{}", faulty_report.unwrap());
            let faulty_report = faulty_report.unwrap();
            let mut new_report = report.clone();
            new_report.remove(faulty_report);
            if is_safe(&new_report) {
                sum += 1;
            }
        }
    }
    Ok(sum)
}

#[derive(PartialEq)]
enum Trend {
    Increasing,
    Decreasing,
    Constant,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_safe_results() {
        let reports = read_file("resources/test_data.txt".to_string());
        let result = calc_safe_results(reports.unwrap());
        assert_eq!(result.unwrap(), 2);
    }
}
