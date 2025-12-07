use crate::state::State;
use shared::{AdventError, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod state;

fn main() -> Result<()> {
    let path = Path::new("day07/resources/input.txt");
    run_part_1(path)?;
    Ok(())
}

fn run_part_1(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let lines = parse_lines(reader)?;
    let total_splits = count_splits(&lines);
    println!("Part 1 - Total Splits: {total_splits}");
    Ok(())
}

fn parse_lines<R: BufRead>(reader: R) -> Result<Vec<Vec<State>>> {
    let mut lines = Vec::new();
    for line in reader.lines() {
        let Ok(line) = line else {
            return Err(AdventError::new("Could not read the next input line"));
        };
        let line = line.trim_end_matches('\r');
        let mut states = Vec::new();
        for next in line.chars() {
            let Some(state) = State::parse(next, false) else {
                return Err(AdventError::new("Encountered unknown state"));
            };
            states.push(state);
        }
        lines.push(states);
    }
    Ok(lines)
}

fn count_splits(lines: &Vec<Vec<State>>) -> u32 {
    let Some(mut current_line) = lines.first().cloned() else {
        return 0;
    };
    let mut total_splits = 0u32;
    for next_index in 1..lines.len() {
        let mut next_line: Vec<State> = lines[next_index].iter().cloned().collect();
        for state_index in 0..current_line.len() {
            let state = current_line[state_index];
            match state {
                State::Beam | State::Start => {
                    let next_state = &mut next_line[state_index];
                    match *next_state {
                        State::Space => *next_state = State::Beam,
                        State::Splitter => {
                            let mut split = false;
                            if state_index > 0 {
                                let next_left_state = &mut next_line[state_index - 1];
                                if *next_left_state == State::Space {
                                    *next_left_state = State::Beam;
                                }
                                split = true;
                            }
                            if state_index + 1 < next_line.len() {
                                let next_right_state = &mut next_line[state_index + 1];
                                if *next_right_state == State::Space {
                                    *next_right_state = State::Beam;
                                }
                                split = true;
                            }
                            if split {
                                total_splits += 1;
                            }
                        }
                        State::Start | State::Beam => {}
                    }
                }
                State::Splitter | State::Space => {}
            }
        }
        current_line = next_line;
    }
    total_splits
}

#[cfg(test)]
mod tests {
    use crate::{count_splits, parse_lines};
    use std::io::Cursor;

    #[test]
    fn test_part1_example() {
        let cursor = create_example_cursor();
        let lines = parse_lines(cursor).unwrap();
        let total_splits = count_splits(&lines);
        assert_eq!(21, total_splits);
    }

    fn create_example_cursor() -> Cursor<String> {
        let lines = [
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];
        let joined = lines.join("\n");
        Cursor::new(joined)
    }
}
