mod advent_error;
mod fresh_range;

use crate::advent_error::AdventError;
use crate::fresh_range::FreshRange;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), AdventError> {
    let path = Path::new("day05/resources/input.txt");
    run_first_part(path)?;
    run_second_part(path)?;
    Ok(())
}

fn run_first_part(path: &Path) -> Result<(), AdventError> {
    let (fresh_ranges, available_ids) = read_ingredients(path, true)?;
    let fresh_count = count_fresh_ingredients(&fresh_ranges, &available_ids);
    println!("Part 1 - Found {fresh_count} fresh ingredient(s)");
    Ok(())
}

fn run_second_part(path: &Path) -> Result<(), AdventError> {
    let (fresh_ranges, _) = read_ingredients(path, false)?;
    let fresh_count = count_all_fresh_ingredients(&fresh_ranges);
    println!("Part 2 - Found {fresh_count} possible fresh ingredient(s)");
    Ok(())
}

fn read_ingredients(
    path: &Path,
    include_available_ids: bool,
) -> Result<(Vec<FreshRange>, Vec<u64>), AdventError> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    read_ingredients_direct(reader, include_available_ids)
}

fn read_ingredients_direct<R: BufRead>(
    reader: R,
    include_available_ids: bool,
) -> Result<(Vec<FreshRange>, Vec<u64>), AdventError> {
    let mut fresh_ranges = Vec::new();
    let mut available_ids = Vec::new();
    let mut is_available_section = false;
    for line in reader.lines() {
        let Ok(line) = line else {
            return Err(AdventError::new("Could not read the next line"));
        };
        let line = line.trim_ascii_end();
        if is_available_section {
            let Ok(available_id) = line.parse::<u64>() else {
                return Err(AdventError::new("Encountered an invalid available ID"));
            };
            available_ids.push(available_id);
        } else {
            if line.is_empty() {
                if include_available_ids {
                    is_available_section = true;
                    continue;
                }
                break;
            }
            let fresh_range = parse_fresh_range(line)?;
            fresh_ranges.push(fresh_range);
        }
    }
    Ok((fresh_ranges, available_ids))
}

fn parse_fresh_range(line: &str) -> Result<FreshRange, AdventError> {
    let Some((first, second)) = line.split_once('-') else {
        return Err(AdventError::new("A range did not contain two values"));
    };
    let Ok(start) = first.parse::<u64>() else {
        return Err(AdventError::new("The first value was not a valid integer"));
    };
    let Ok(end) = second.parse::<u64>() else {
        return Err(AdventError::new("The second value was not a valid integer"));
    };
    let range = FreshRange { start, end };
    Ok(range)
}

fn count_fresh_ingredients(fresh_ranges: &[FreshRange], available_ids: &Vec<u64>) -> u64 {
    let mut fresh_count = 0u64;
    for available_id in available_ids {
        for fresh_range in fresh_ranges {
            if fresh_range.contains(*available_id) {
                fresh_count += 1;
                break;
            }
        }
    }
    fresh_count
}

fn count_all_fresh_ingredients(fresh_ranges: &[FreshRange]) -> u64 {
    let merged_ranges = merge_ranges(fresh_ranges);
    let mut fresh_count = 0u64;
    for fresh_range in merged_ranges {
        fresh_count += fresh_range.count();
    }
    fresh_count
}

fn merge_ranges(fresh_range: &[FreshRange]) -> Vec<FreshRange> {
    let mut merged_ranges = Vec::new();
    let mut removed = vec![false; fresh_range.len()];
    for (index, range) in fresh_range.iter().enumerate() {
        if removed[index] {
            continue;
        }
        let mut new_range = *range;
        loop {
            // If we ever successfully merge with another range, it's possible
            // there was a previous range we skipped because our ranges didn't
            // previously overlap, but now they will. So we start back over at
            // the beginning and try all over again, repeating as many times as
            // necessary.
            let mut merged = false;
            for other_index in (index + 1)..fresh_range.len() {
                if removed[other_index] {
                    continue;
                }
                let other_range = fresh_range[other_index];
                if let Some(merged_range) = new_range.try_merge(other_range) {
                    new_range = merged_range;
                    removed[other_index] = true;
                    merged = true;
                }
            }
            if !merged {
                break;
            }
        }
        merged_ranges.push(new_range);
        removed[index] = true;
    }
    merged_ranges
}

#[cfg(test)]
mod tests {
    use crate::{count_all_fresh_ingredients, count_fresh_ingredients, read_ingredients_direct};
    use std::io::Cursor;

    #[test]
    fn test_part1_example() {
        let cursor = create_example_cursor();
        let (ranges, ids) = read_ingredients_direct(cursor, true).unwrap();
        let fresh_count = count_fresh_ingredients(&ranges, &ids);
        assert_eq!(3, fresh_count);
    }

    #[test]
    fn test_part2_example() {
        let cursor = create_example_cursor();
        let (ranges, _) = read_ingredients_direct(cursor, false).unwrap();
        let fresh_count = count_all_fresh_ingredients(&ranges);
        assert_eq!(14, fresh_count);
    }

    fn create_example_cursor() -> Cursor<&'static str> {
        const EXAMPLE_INPUT: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        Cursor::new(EXAMPLE_INPUT)
    }
}
