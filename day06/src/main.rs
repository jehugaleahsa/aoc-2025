mod column;
mod operator;

use crate::column::Column;
use crate::operator::Operator;
use shared::AdventError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), AdventError> {
    let path = Path::new("day06/resources/input.txt");
    run_first_part(path)?;
    run_second_part(path)?;
    Ok(())
}

fn run_first_part(path: &Path) -> Result<(), AdventError> {
    let columns = read_columns(path)?;
    let total = sum_results(&columns);
    println!("Part 1 - Total {total}");
    Ok(())
}

fn read_columns(path: &Path) -> Result<Vec<Column>, AdventError> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    read_columns_direct(reader)
}

fn read_columns_direct<R: BufRead>(reader: R) -> Result<Vec<Column>, AdventError> {
    let mut columns = Vec::new();
    let mut is_operators = false;
    for line in reader.lines() {
        let Ok(line) = line else {
            return Err(AdventError::new("Could not read the next input line"));
        };
        let values = line.split_ascii_whitespace();
        for (index, value) in values.enumerate() {
            if index == columns.len() {
                columns.push(Column::default());
            }
            let column = &mut columns[index];
            if is_operators {
                let Some(operator) = Operator::parse_str(value) else {
                    return Err(AdventError::new("Encountered an invalid operator"));
                };
                column.operator = operator;
            } else {
                if let Ok(number) = value.parse::<i64>() {
                    column.values.push(number);
                } else if index == 0
                    && let Some(operator) = Operator::parse_str(value)
                {
                    is_operators = true;
                    column.operator = operator;
                } else {
                    return Err(AdventError::new(
                        "Encountered an invalid number or operator",
                    ));
                }
            }
        }
    }
    Ok(columns)
}

fn run_second_part(path: &Path) -> Result<(), AdventError> {
    let columns = read_columns_hard(path)?;
    let total = sum_results(&columns);
    println!("Part 2 - Total {total}");
    Ok(())
}

fn sum_results(columns: &Vec<Column>) -> i64 {
    let mut total = 0i64;
    for column in columns {
        let sub_total = column.fold();
        total += sub_total;
    }
    total
}

fn read_columns_hard(path: &Path) -> Result<Vec<Column>, AdventError> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    read_columns_hard_direct(reader)
}

fn read_columns_hard_direct<R: BufRead>(reader: R) -> Result<Vec<Column>, AdventError> {
    // Grab all the lines
    let mut lines = Vec::new();
    for line in reader.lines() {
        let Ok(line) = line else {
            return Err(AdventError::new(""));
        };
        let line = line.trim_end_matches('\r').to_string();
        lines.push(line);
    }

    // Determine the operators and the indexes of each column
    let Some(operator_line) = lines.last() else {
        return Ok(Vec::new());
    };
    let mut operators = Vec::new();
    let mut chunk_ranges = Vec::new();
    let mut previous_index = 0usize;
    for (index, value) in operator_line.chars().enumerate() {
        if let Some(operator) = Operator::parse(value) {
            operators.push(operator);
            if index != 0 {
                let range = previous_index..(index - 1); // Extra blank column
                chunk_ranges.push(range);
                previous_index = index;
            }
        }
    }
    chunk_ranges.push(previous_index..operator_line.len());
    debug_assert!(
        operators.len() == chunk_ranges.len(),
        "Each chunk should have an operator"
    );

    // Break each line into column chunks
    let mut chunked_lines = Vec::with_capacity(lines.len() - 1);
    for line in lines.iter().take(lines.len() - 1) {
        let mut chunked_line = Vec::with_capacity(operators.len());
        for chunk_range in chunk_ranges.iter().cloned() {
            let substring = &line[chunk_range];
            chunked_line.push(substring);
        }
        chunked_lines.push(chunked_line);
    }

    // Grab the values down each chunk's columns
    let mut columns = Vec::new();
    for index in 0..chunk_ranges.len() {
        let range = chunk_ranges[index].clone();
        let operator = operators[index];
        let mut column = Column::default();
        column.operator = operator;
        column.values.resize(range.len(), 0);
        for line in lines.iter().take(lines.len() - 1) {
            let chunk: Vec<char> = line[range.clone()].chars().collect();
            for index in 0..range.len() {
                let value = &mut column.values[index];
                let next = chunk[index];
                if next != ' ' {
                    let Some(parsed_value) = next.to_digit(10) else {
                        return Err(AdventError::new("Encountered an invalid number"));
                    };
                    *value *= 10;
                    *value += parsed_value as i64;
                }
            }
        }
        columns.push(column);
    }
    Ok(columns)
}

#[cfg(test)]
mod tests {
    use crate::{read_columns_hard_direct, sum_results};
    use std::io::Cursor;

    #[test]
    fn test_part2_example() {
        let cursor = create_cursor();
        let columns = read_columns_hard_direct(cursor).unwrap();
        assert_eq!(8_544, columns[0].fold());
        assert_eq!(625, columns[1].fold());
        assert_eq!(3_253_600, columns[2].fold());
        assert_eq!(1_058, columns[3].fold());
        let total = sum_results(&columns);
        assert_eq!(3_263_827, total);
    }

    fn create_cursor() -> Cursor<String> {
        let lines = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];
        Cursor::new(lines.join("\n"))
    }
}
