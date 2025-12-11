use crate::state::State;
use shared::{AdventError, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod state;

fn main() -> Result<()> {
    let path = Path::new("day07/resources/input.txt");
    run_part_1(path)?;
    run_part_2(path)?;
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

fn count_splits(lines: &[Vec<State>]) -> u32 {
    let Some(mut current_line) = lines.first().cloned() else {
        return 0;
    };
    let mut total_splits = 0u32;
    for next_index in 1..lines.len() {
        let mut next_line: Vec<State> = lines[next_index].clone();
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

fn run_part_2(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let lines = parse_lines(reader)?;
    let total_timelines = count_timelines(&lines);
    println!("Part 2 - Total timelines: {total_timelines}");
    Ok(())
}

fn count_timelines(lines: &Vec<Vec<State>>) -> u64 {
    let Some(current_line) = lines.first() else {
        return 0;
    };
    let beam_index = current_line
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, s)| *s == State::Start)
        .map(|(ix, _)| ix)
        .next();
    let Some(beam_index) = beam_index else {
        return 0;
    };
    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
    count_alternate_timeline_splits(lines, 1, beam_index, &mut cache) + 1 // Include initial timeline!
}

fn count_alternate_timeline_splits(
    lines: &Vec<Vec<State>>,
    current_index: usize,
    beam_index: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let Some(current_line) = lines.get(current_index) else {
        return 0;
    };
    if let Some(total) = cache.get(&(current_index, beam_index)) {
        return *total;
    }
    let state = current_line[beam_index];
    let count = match state {
        State::Space => {
            count_alternate_timeline_splits(lines, current_index + 1, beam_index, cache)
        }
        State::Splitter => {
            let next_index = current_index + 1;
            let left =
                count_propagated_beam_paths(lines, next_index, beam_index, Ordering::Less, cache);
            let right = count_propagated_beam_paths(
                lines,
                next_index,
                beam_index,
                Ordering::Greater,
                cache,
            );
            left + right + 1
        }
        State::Beam | State::Start => 0,
    };
    cache.insert((current_index, beam_index), count);
    count
}

fn count_propagated_beam_paths(
    lines: &Vec<Vec<State>>,
    next_index: usize,
    state_index: usize,
    ordering: Ordering,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let Some(next_line) = lines.get(next_index) else {
        return 0;
    };
    let beam_index = match ordering {
        Ordering::Equal => state_index,
        Ordering::Less => {
            if state_index == 0 {
                return 0;
            }
            state_index - 1
        }
        Ordering::Greater => {
            let right_index = state_index + 1;
            if right_index >= next_line.len() {
                return 0;
            }
            right_index
        }
    };
    count_alternate_timeline_splits(lines, next_index, beam_index, cache)
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

#[cfg(test)]
mod tests {
    use crate::{count_splits, count_timelines, parse_lines};
    use std::io::Cursor;

    #[test]
    fn test_part1_example() {
        let cursor = create_example_cursor();
        let lines = parse_lines(cursor).unwrap();
        let total_splits = count_splits(&lines);
        assert_eq!(21, total_splits);
    }

    #[test]
    fn test_part2_example() {
        let cursor = create_example_cursor();
        let lines = parse_lines(cursor).unwrap();
        let total_timelines = count_timelines(&lines);
        assert_eq!(40, total_timelines);
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
