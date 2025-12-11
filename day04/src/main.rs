use shared::AdventError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), AdventError> {
    let path = Path::new("day04/resources/input.txt");
    run_first_part(path)?;
    run_second_part(path)?;
    Ok(())
}

fn run_first_part(path: &Path) -> Result<(), AdventError> {
    let rolls = read_roll_lines(path)?;
    let accessible_rolls = count_accessible_rolls(&rolls);
    println!("Part 1 - There are {accessible_rolls} accessible roll(s)");
    Ok(())
}

fn count_accessible_rolls(rolls: &[Vec<bool>]) -> u32 {
    let mut accessible_rolls = 0u32;
    for (row_index, row) in rolls.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if !*column {
                continue;
            }
            let occupied_count = count_adjacent_rolls(rolls, row_index, column_index);
            if occupied_count < 4 {
                accessible_rolls += 1;
            }
        }
    }
    accessible_rolls
}

fn run_second_part(path: &Path) -> Result<(), AdventError> {
    let mut rolls = read_roll_lines(path)?;
    let moved_rolls = count_accessible_rolls_repeatedly(&mut rolls);
    println!("Part 2 - There are {moved_rolls} moved roll(s)");
    Ok(())
}

fn count_accessible_rolls_repeatedly(rolls: &mut [Vec<bool>]) -> usize {
    let mut moved_rolls = 0usize;
    loop {
        let accessible_rolls = find_accessible_rolls(rolls);
        let accessible_roll_count = accessible_rolls.len();
        if accessible_roll_count == 0 {
            break;
        }
        moved_rolls += accessible_roll_count;
        for (row_index, column_index) in accessible_rolls {
            rolls[row_index][column_index] = false;
        }
    }
    moved_rolls
}

fn find_accessible_rolls(rolls: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let mut accessible_rolls = Vec::new();
    for (row_index, row) in rolls.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if !*column {
                continue;
            }
            let occupied_count = count_adjacent_rolls(rolls, row_index, column_index);
            if occupied_count < 4 {
                accessible_rolls.push((row_index, column_index));
            }
        }
    }
    accessible_rolls
}

fn count_adjacent_rolls(rolls: &[Vec<bool>], row_index: usize, column_index: usize) -> u32 {
    let row = &rolls[row_index];
    let mut occupied_count = 0u32;
    let start_row = row_index as isize - 1;
    let end_row = row_index as isize + 1;
    let start_column = column_index as isize - 1;
    let end_column = column_index as isize + 1;
    for adjacent_row_index in start_row..=end_row {
        // Ignore rows that are out of bounds.
        if adjacent_row_index < 0 || adjacent_row_index >= rolls.len() as isize {
            continue;
        }
        for adjacent_column_index in start_column..=end_column {
            // Ignore columns that are out of bounds.
            if adjacent_column_index < 0 || adjacent_column_index >= row.len() as isize {
                continue;
            }
            // Don't look at the current value.
            if adjacent_row_index == row_index as isize
                && adjacent_column_index == column_index as isize
            {
                continue;
            }
            if rolls[adjacent_row_index as usize][adjacent_column_index as usize] {
                occupied_count += 1;
            }
        }
    }
    occupied_count
}

fn read_roll_lines(path: &Path) -> Result<Vec<Vec<bool>>, AdventError> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    read_roll_lines_direct(reader)
}

fn read_roll_lines_direct<R: BufRead>(reader: R) -> Result<Vec<Vec<bool>>, AdventError> {
    let mut rolls = Vec::new();
    for line in reader.lines() {
        let Ok(line) = line else {
            return Err(AdventError::new("Could not read the next line"));
        };
        let mut roll_line = Vec::new();
        for next in line.chars() {
            let is_roll = match next {
                '.' => false,
                '@' => true,
                _ => return Err(AdventError::new("Encountered an invalid character")),
            };
            roll_line.push(is_roll);
        }
        rolls.push(roll_line);
    }
    Ok(rolls)
}

#[cfg(test)]
mod tests {
    use crate::{
        count_accessible_rolls, count_accessible_rolls_repeatedly, read_roll_lines_direct,
    };
    use std::io::Cursor;

    #[test]
    fn test_part1_example() {
        let rolls = read_test_data();
        let count = count_accessible_rolls(&rolls);
        assert_eq!(13, count);
    }

    #[test]
    fn test_part2_example() {
        let mut rolls = read_test_data();
        let count = count_accessible_rolls_repeatedly(&mut rolls);
        assert_eq!(43, count);
    }

    fn read_test_data() -> Vec<Vec<bool>> {
        let raw_data = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let cursor = Cursor::new(raw_data);
        read_roll_lines_direct(cursor).unwrap()
    }
}
