use shared::AdventError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), AdventError> {
    let path = Path::new("day03/resources/input.txt");
    run_first_part(path)?;
    run_second_part(path)?;
    Ok(())
}

fn run_first_part(path: &Path) -> Result<(), AdventError> {
    let file = File::open(path).map_err(|_| AdventError::new("Failed to open the file"))?;
    let reader = BufReader::new(file);

    let mut total = 0u64;
    for line in reader.lines() {
        let line = line.map_err(|_| AdventError::new("Could not read the next line"))?;
        let line = line.trim_end_matches('\r');
        total += process_line_part_1(line)?;
    }
    println!("First part - Total joltage: {total}");
    Ok(())
}

fn process_line_part_1(line: &str) -> Result<u64, AdventError> {
    let mut top_values: Vec<u32> = Vec::with_capacity(2);
    let line: Vec<char> = line.chars().collect();
    let line_length = line.len();
    for (index, next) in line.into_iter().enumerate() {
        let digit = char::to_digit(next, 10)
            .ok_or_else(|| AdventError::new("Encountered an invalid digit"))?;
        match top_values.len() {
            0 => top_values.push(digit),
            1 => {
                if digit > top_values[0] {
                    if index + 1 < line_length {
                        top_values[0] = digit;
                    } else {
                        top_values.push(digit);
                    }
                } else {
                    top_values.push(digit);
                }
            }
            2 => {
                if digit > top_values[0] {
                    // Any time the next digit is larger than the leading digit, as long as
                    // there's a follow-up digit, the resulting value will be larger. We assume
                    // the number of characters
                    if index + 1 < line_length {
                        top_values[0] = digit;
                        top_values.resize(1, 0);
                    } else {
                        if top_values[1] > top_values[0] {
                            top_values[0] = top_values[1];
                        }
                        top_values[1] = digit;
                    }
                } else if digit > top_values[1] {
                    top_values[1] = digit;
                }
            }
            _ => return Err(AdventError::new("Encountered more than 2 top values."))?,
        }
    }
    if top_values.len() != 2 {
        return Err(AdventError::new(
            "A line did not contain at least two values.",
        ));
    }
    let sub_total = top_values
        .into_iter()
        .map(u64::from)
        .fold(0, |acc, next| acc * 10 + next);
    Ok(sub_total)
}

fn run_second_part(path: &Path) -> Result<(), AdventError> {
    let file = File::open(path).map_err(|_| AdventError::new("Failed to open the file"))?;
    let reader = BufReader::new(file);

    let mut total = 0u64;
    for line in reader.lines() {
        let line = line.map_err(|_| AdventError::new("Could not read the next line"))?;
        let line = line.trim_end_matches('\r');
        total += process_line_part_2(line, 12)?;
    }
    println!("Second part - Total joltage: {total}");
    Ok(())
}

fn process_line_part_2(line: &str, battery_count: usize) -> Result<u64, AdventError> {
    let mut top_values: Vec<u32> = Vec::with_capacity(battery_count);
    let line: Vec<char> = line.chars().collect();
    let line_length = line.len();
    for (index, next) in line.into_iter().enumerate() {
        let Some(digit) = char::to_digit(next, 10) else {
            return Err(AdventError::new("Encountered an invalid digit"));
        };
        match top_values.len() {
            x if x <= battery_count => {
                let mut done = false;
                for value_index in 0..top_values.len() {
                    let value = top_values[value_index];
                    if digit > value {
                        // Any time the next digit is larger than the current digit, as long as
                        // there's enough follow-up digits, the resulting value will be larger.
                        if index + battery_count - (value_index + 1) < line_length {
                            top_values[value_index] = digit;
                            top_values.resize(value_index + 1, 0);
                            done = true;
                            break;
                        }
                    }
                }
                if !done && top_values.len() < battery_count {
                    top_values.push(digit);
                }
            }
            _ => {
                Err(AdventError::new(format!(
                    "Encountered more than {battery_count} top value(s)."
                )))?;
            }
        }
    }
    if top_values.len() != battery_count {
        return Err(AdventError::new(format!(
            "A line did not contain at least {battery_count} value(s)."
        )));
    }
    let sub_total = top_values
        .into_iter()
        .map(u64::from)
        .fold(0, |acc, next| acc * 10 + next);
    Ok(sub_total)
}

#[cfg(test)]
mod tests {
    use crate::{process_line_part_1, process_line_part_2};

    #[test]
    fn test_run_part1_example1() {
        let total = process_line_part_1("987654321111111").unwrap();
        assert_eq!(98, total);
    }

    #[test]
    fn test_run_part1_example2() {
        let total = process_line_part_1("811111111111119").unwrap();
        assert_eq!(89, total);
    }

    #[test]
    fn test_run_part1_example3() {
        let total = process_line_part_1("234234234234278").unwrap();
        assert_eq!(78, total);
    }

    #[test]
    fn test_run_part1_example4() {
        let total = process_line_part_1("818181911112111").unwrap();
        assert_eq!(92, total);
    }

    #[test]
    fn test_run_part2_example1() {
        let total = process_line_part_2("987654321111111", 12).unwrap();
        assert_eq!(987_654_321_111, total);
    }

    #[test]
    fn test_run_part2_example2() {
        let total = process_line_part_2("811111111111119", 12).unwrap();
        assert_eq!(811_111_111_119, total);
    }

    #[test]
    fn test_run_part2_example3() {
        let total = process_line_part_2("234234234234278", 12).unwrap();
        assert_eq!(434_234_234_278, total);
    }

    #[test]
    fn test_run_part2_example4() {
        let total = process_line_part_2("818181911112111", 12).unwrap();
        assert_eq!(888_911_112_111, total);
    }
}
