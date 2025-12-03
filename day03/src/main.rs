use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct AdventError {
    message: String,
}

impl AdventError {
    #[inline]
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for AdventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AdventError {}

fn main() -> Result<(), AdventError> {
    let path = Path::new("day03/resources/input.txt");
    run_first_part(path)?;
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
        .map(|x| u64::from(x))
        .fold(0, |acc, next| acc * 10 + next);
    Ok(sub_total)
}

#[cfg(test)]
mod tests {
    use crate::process_line_part_1;

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
}
