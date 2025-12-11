use shared::AdventError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), AdventError> {
    let path = Path::new("day02/resources/input.txt");
    run_first_part(path)?;
    run_second_part(path)?;
    Ok(())
}

fn run_first_part(path: &Path) -> Result<(), AdventError> {
    let file = File::open(path).map_err(|_| AdventError::new("Failed to open the file"))?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();
    let mut total_id_count = 0u64;
    loop {
        buffer.clear();
        let read = reader
            .read_until(b',', &mut buffer)
            .map_err(|_| AdventError::new("Could not read the next value"))?;
        if read == 0 {
            break;
        }
        let (start, end) = extract_start_end(&buffer)?;
        total_id_count += total_invalid_ids(start, end)?;
    }

    println!("Part 1 - Invalid ID count: {total_id_count}");

    Ok(())
}

fn run_second_part(path: &Path) -> Result<(), AdventError> {
    let file = File::open(path).map_err(|_| AdventError::new("Failed to open the file"))?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();
    let mut total_id_count = 0u64;
    loop {
        buffer.clear();
        let read = reader
            .read_until(b',', &mut buffer)
            .map_err(|_| AdventError::new("Could not read the next value"))?;
        if read == 0 {
            break;
        }
        let (start, end) = extract_start_end(&buffer)?;
        total_id_count += total_invalid_ids_part_2(start, end)?;
    }

    println!("Part 2 - Invalid ID count: {total_id_count}");

    Ok(())
}

fn extract_start_end(buffer: &[u8]) -> Result<(i64, i64), AdventError> {
    let split: Vec<&[u8]> = buffer.splitn(2, |b| *b == b'-').collect();
    if split.len() != 2 {
        return Err(AdventError::new("A range did not contain 2 parts"));
    }
    let start = str::from_utf8(split[0])
        .map_err(|_| AdventError::new("The start of the range was not a valid string"))?;
    let end = str::from_utf8(split[1])
        .map_err(|_| AdventError::new("The end of the range was not a valid string"))?;
    let end = end.trim_end_matches([',', '\r', '\n']);
    let start = start.parse().map_err(|_| {
        AdventError::new(format!(
            "The start of the range was not a valid integer: {start}"
        ))
    })?;
    let end = end.parse().map_err(|_| {
        AdventError::new(format!(
            "The end of the range was not a valid integer: {end}"
        ))
    })?;
    Ok((start, end))
}

fn total_invalid_ids(start: i64, end: i64) -> Result<u64, AdventError> {
    let mut invalid_id_total = 0u64;
    for value in start..=end {
        let value_str = value.to_string();
        let mid = value_str.len() / 2;
        let (prefix, suffix) = value_str
            .split_at_checked(mid)
            .ok_or_else(|| AdventError::new("Could not find the middle of the value string"))?;
        if prefix == suffix {
            invalid_id_total += u64::try_from(value)
                .map_err(|_| AdventError::new("Encountered a negative value"))?;
        }
    }
    Ok(invalid_id_total)
}

fn total_invalid_ids_part_2(start: i64, end: i64) -> Result<u64, AdventError> {
    let mut invalid_id_total = 0u64;
    for value in start..=end {
        let chars: Vec<char> = value.to_string().chars().collect();
        let mid = chars.len() / 2;
        for chunk_size in (1..=mid).rev() {
            if !chars.len().is_multiple_of(chunk_size) {
                continue;
            }
            let chunks_1 = chars.chunks(chunk_size);
            let chunks_2 = chars.chunks(chunk_size).skip(1);
            let all_equal = chunks_1.zip(chunks_2).all(|(x, y)| x == y);
            if all_equal {
                invalid_id_total += u64::try_from(value)
                    .map_err(|_| AdventError::new("Encountered a negative value"))?;
                break;
            }
        }
    }
    Ok(invalid_id_total)
}

#[cfg(test)]
mod tests {
    use crate::{total_invalid_ids, total_invalid_ids_part_2};

    #[test]
    fn test_part1_11_22() {
        let total = total_invalid_ids(11, 22).unwrap();
        assert_eq!(33, total); // 11 and 22
    }

    #[test]
    fn test_part2_11_22() {
        let total = total_invalid_ids_part_2(11, 22).unwrap();
        assert_eq!(11 + 22, total);
    }

    #[test]
    fn test_part1_95_115() {
        let total = total_invalid_ids(95, 115).unwrap();
        assert_eq!(9_9, total);
    }

    #[test]
    fn test_part2_95_115() {
        let total = total_invalid_ids_part_2(95, 115).unwrap();
        assert_eq!(99 + 111, total);
    }

    #[test]
    fn test_part1_998_1012() {
        let total = total_invalid_ids(998, 1012).unwrap();
        assert_eq!(10_10, total);
    }

    #[test]
    fn test_part2_998_1012() {
        let total = total_invalid_ids_part_2(998, 1012).unwrap();
        assert_eq!(999 + 1010, total);
    }

    #[test]
    fn test_part1_1188511880_1188511890() {
        let total = total_invalid_ids(1_188_511_880, 1_188_511_890).unwrap();
        #[allow(clippy::large_digit_groups)] {
            assert_eq!(11885_11885, total);
        }
    }

    #[test]
    fn test_part2_1188511880_1188511890() {
        let total = total_invalid_ids_part_2(1_188_511_880, 1_188_511_890).unwrap();
        #[allow(clippy::large_digit_groups)] {
            assert_eq!(11885_11885, total);
        }
    }

    #[test]
    fn test_part1_222220_222224() {
        let total = total_invalid_ids(222_220, 222_224).unwrap();
        assert_eq!(222_222, total);
    }

    #[test]
    fn test_part2_222220_222224() {
        let total = total_invalid_ids_part_2(222_220, 222_224).unwrap();
        #[allow(clippy::unreadable_literal)] {
            assert_eq!(222222, total); // Multiple valid splits... counted only once!
        }
    }

    #[test]
    fn test_part1_1698522_1698528() {
        let total = total_invalid_ids(1_698_522, 1_698_528).unwrap();
        assert_eq!(0, total);
    }

    #[test]
    fn test_part2_1698522_1698528() {
        let total = total_invalid_ids_part_2(1_698_522, 1_698_528).unwrap();
        assert_eq!(0, total);
    }

    #[test]
    fn test_part1_446443_446449() {
        let total = total_invalid_ids(446_443, 446_449).unwrap();
        assert_eq!(446_446, total);
    }

    #[test]
    fn test_part2_446443_446449() {
        let total = total_invalid_ids_part_2(446_443, 446_449).unwrap();
        assert_eq!(446_446, total);
    }

    #[test]
    fn test_part1_38593856_38593862() {
        let total = total_invalid_ids(38_593_856, 38_593_862).unwrap();
        assert_eq!(3859_3859, total);
    }

    #[test]
    fn test_part2_38593856_38593862() {
        let total = total_invalid_ids_part_2(38_593_856, 38_593_862).unwrap();
        assert_eq!(3859_3859, total);
    }

    #[test]
    fn test_part2_565653_565659() {
        let total = total_invalid_ids_part_2(565_653, 565_659).unwrap();
        assert_eq!(565_656, total);
    }

    #[test]
    fn test_part2_824824821_824824827() {
        let total = total_invalid_ids_part_2(824_824_821, 824_824_827).unwrap();
        assert_eq!(824_824_824, total);
    }

    #[test]
    fn test_part2_2121212118_2121212124() {
        let total = total_invalid_ids_part_2(2_121_212_118, 2_121_212_124).unwrap();
        assert_eq!(21_21_21_21_21, total);
    }
}
