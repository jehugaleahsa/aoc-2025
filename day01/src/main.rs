use shared::AdventError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), AdventError> {
    let path = Path::new("resources/input.txt");
    run_first_part(&path)?;
    run_second_part(&path)?;
    Ok(())
}

fn run_first_part(path: &Path) -> Result<(), AdventError> {
    let file = File::open(path).map_err(|_| AdventError::new("Failed to open the file"))?;
    let reader = BufReader::new(file);

    let mut zeros = 0i32;
    let mut value = 50i32;
    for line in reader.lines() {
        let line = line.map_err(|_| AdventError::new("Failed to read the next line"))?;
        let amount = parse_amount(line)?;
        value += amount;
        value %= 100;
        if value == 0 {
            zeros += 1;
        } else if value < 0 {
            value += 100;
        }
    }
    println!("Zero count (first): {zeros}");
    Ok(())
}

fn run_second_part(path: &Path) -> Result<(), AdventError> {
    let file = File::open(path).map_err(|_| AdventError::new("Failed to open the file"))?;
    let reader = BufReader::new(file);

    let mut zeros = 0i32;
    let mut value = 50i32;
    for line in reader.lines() {
        let line = line.map_err(|_| AdventError::new("Failed to read the next line"))?;
        let amount = parse_amount(line)?;

        let mut full_rotations = amount / 100;
        if full_rotations < 0 {
            full_rotations = -full_rotations;
        }
        let partial_rotation = amount % 100;
        let mut new_value = value + partial_rotation;
        if new_value == 0 {
            full_rotations += 1;
        } else if new_value < 0 {
            if value != 0 {
                full_rotations += 1;
            }
            new_value += 100;
        } else if new_value >= 100 {
            if value != 0 {
                full_rotations += 1;
            }
            new_value -= 100;
        }
        zeros += full_rotations;
        value = new_value;
    }
    println!("Zero count (second) (fast): {zeros}");
    Ok(())
}

fn parse_amount(line: String) -> Result<i32, AdventError> {
    let (direction, rest) = line
        .split_at_checked(1)
        .ok_or_else(|| AdventError::new("Encountered an invalid rotation"))?;
    let negative = match direction {
        "L" => true,
        "R" => false,
        _ => {
            return Err(AdventError::new(
                "Encountered an invalid rotation direction",
            ));
        }
    };
    let mut amount = rest
        .parse::<i32>()
        .map_err(|_| AdventError::new("Could not parse the rotation amount"))?;
    if negative {
        amount = -amount;
    }
    Ok(amount)
}
