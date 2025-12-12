use shared::{AdventError, Result};

#[derive(Debug)]
pub struct Present {
    id: u32,
    shape: Vec<Vec<bool>>,
    row_count: usize,
    column_count: usize,
}

impl Present {
    #[inline]
    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[inline]
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    #[inline]
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.column_count
    }

    #[inline]
    #[must_use]
    pub fn is_set(&self, row_index: usize, column_index: usize) -> bool {
        self.shape[row_index][column_index]
    }

    #[must_use]
    pub fn rotate_clockwise(&self) -> Self {
        let id = self.id;
        let mut shape = Vec::new();
        let new_row_count = self.column_count;
        let new_column_count = self.row_count;
        for row_index in 0..new_row_count {
            let mut new_row = Vec::new();
            for column_index in 0..new_column_count {
                let old_row_index = self.row_count - column_index - 1;
                let value = self.shape[old_row_index][row_index];
                new_row.push(value);
            }
            shape.push(new_row);
        }
        Self {
            id,
            shape,
            row_count: new_row_count,
            column_count: new_column_count,
        }
    }

    #[must_use]
    pub fn flip_vertically(&self) -> Self {
        let id = self.id;
        let mut shape = Vec::new();
        for row_index in 0..self.row_count {
            let mut new_row = Vec::new();
            for column_index in 0..self.column_count {
                let old_row_index = self.row_count - row_index - 1;
                let value = self.shape[old_row_index][column_index];
                new_row.push(value);
            }
            shape.push(new_row);
        }
        Self {
            id,
            shape,
            row_count: self.row_count,
            column_count: self.column_count,
        }
    }

    #[must_use]
    pub fn flip_horizontally(&self) -> Self {
        let id = self.id;
        let mut shape = Vec::new();
        for row_index in 0..self.row_count {
            let mut new_row = Vec::new();
            for column_index in 0..self.column_count {
                let old_column_index = self.column_count - column_index - 1;
                let value = self.shape[row_index][old_column_index];
                new_row.push(value);
            }
            shape.push(new_row);
        }
        Self {
            id,
            shape,
            row_count: self.row_count,
            column_count: self.column_count,
        }
    }

    pub fn parse(value: &str) -> Result<Option<(&str, Present)>> {
        let mut value = value;
        let Some(newline_index) = value.find('\n') else {
            return Ok(None);
        };
        let line = &value[..newline_index];
        let line = line.trim_ascii_end();
        let Some(colon_index) = line.find(':') else {
            return Err(AdventError::new(
                "The first line of the present was malformed",
            ));
        };
        if colon_index != line.len() - 1 {
            return Err(AdventError::new(
                "The first line of the present was malformed",
            ));
        }
        let id = &line[..colon_index];
        let Ok(id) = id.parse::<u32>() else {
            return Err(AdventError::new("The present ID was not a valid integer"));
        };
        if newline_index + 1 > value.len() {
            return Err(AdventError::new("The present was missing its shape"));
        }
        value = &value[newline_index + 1..];
        let mut shape = Vec::new();
        let mut column_count = None;
        while !value.is_empty() {
            let newline_index = if let Some(index) = value.find('\n') {
                index
            } else {
                value.len()
            };
            let line = &value[..newline_index];
            let line = line.trim_ascii_end();
            if line.is_empty() {
                if newline_index + 1 > value.len() {
                    value = &value[newline_index..];
                } else {
                    value = &value[newline_index + 1..];
                }
                break;
            }
            let mut row = Vec::new();
            for column in line.chars() {
                let value = match column {
                    '#' => true,
                    '.' => false,
                    _ => return Err(AdventError::new("Encountered an unknown present column")),
                };
                row.push(value);
            }
            if let Some(count) = column_count {
                if row.len() != count {
                    return Err(AdventError::new("Not all rows were the same length"));
                }
            } else {
                column_count = Some(row.len());
            }
            shape.push(row);
            if newline_index + 1 > value.len() {
                value = &value[newline_index..];
            } else {
                value = &value[newline_index + 1..];
            }
        }
        let Some(column_count) = column_count else {
            return Err(AdventError::new("The present contained no rows"));
        };
        let row_count = shape.len();
        let present = Present {
            id,
            shape,
            row_count,
            column_count,
        };
        Ok(Some((value, present)))
    }
}

#[cfg(test)]
mod tests {
    use crate::present::Present;

    #[test]
    fn test_parse() {
        let lines = ["0:", "###", "##.", "##."];
        let joined = lines.join("\n");
        let (left_overs, present) = Present::parse(&joined).unwrap().unwrap();
        assert!(left_overs.is_empty());
        assert_eq!(0, present.id());
        assert!(present.is_set(0, 0));
        assert!(present.is_set(0, 1));
        assert!(present.is_set(0, 2));
        assert!(present.is_set(1, 0));
        assert!(present.is_set(1, 1));
        assert!(!present.is_set(1, 2));
        assert!(present.is_set(2, 0));
        assert!(present.is_set(2, 1));
        assert!(!present.is_set(2, 2));
    }

    #[test]
    fn test_rotate_clockwise() {
        let present = Present {
            id: 0,
            shape: vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, true, false],
            ],
            row_count: 3,
            column_count: 3,
        };
        let rotated = present.rotate_clockwise();
        assert_eq!(0, rotated.id());
        assert_eq!(3, rotated.row_count());
        assert_eq!(3, rotated.column_count());
        assert!(rotated.is_set(0, 0));
        assert!(rotated.is_set(0, 1));
        assert!(rotated.is_set(0, 2));
        assert!(rotated.is_set(1, 0));
        assert!(rotated.is_set(1, 1));
        assert!(rotated.is_set(1, 2));
        assert!(!rotated.is_set(2, 0));
        assert!(!rotated.is_set(2, 1));
        assert!(rotated.is_set(2, 2));
    }

    #[test]
    fn test_flip_vertically() {
        let present = Present {
            id: 0,
            shape: vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, true, false],
            ],
            row_count: 3,
            column_count: 3,
        };
        let rotated = present.flip_vertically();
        assert_eq!(0, rotated.id());
        assert_eq!(3, rotated.row_count());
        assert_eq!(3, rotated.column_count());
        assert!(rotated.is_set(0, 0));
        assert!(rotated.is_set(0, 1));
        assert!(!rotated.is_set(0, 2));
        assert!(rotated.is_set(1, 0));
        assert!(rotated.is_set(1, 1));
        assert!(!rotated.is_set(1, 2));
        assert!(rotated.is_set(2, 0));
        assert!(rotated.is_set(2, 1));
        assert!(rotated.is_set(2, 2));
    }

    #[test]
    fn test_flip_horizontally() {
        let present = Present {
            id: 0,
            shape: vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, true, false],
            ],
            row_count: 3,
            column_count: 3,
        };
        let rotated = present.flip_horizontally();
        assert_eq!(0, rotated.id());
        assert_eq!(3, rotated.row_count());
        assert_eq!(3, rotated.column_count());
        assert!(rotated.is_set(0, 0));
        assert!(rotated.is_set(0, 1));
        assert!(rotated.is_set(0, 2));
        assert!(!rotated.is_set(1, 0));
        assert!(rotated.is_set(1, 1));
        assert!(rotated.is_set(1, 2));
        assert!(!rotated.is_set(2, 0));
        assert!(rotated.is_set(2, 1));
        assert!(rotated.is_set(2, 2));
    }
}
