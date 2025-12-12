use shared::{AdventError, Result};
use std::collections::HashMap;

type PresentId = u32;

#[derive(Debug)]
pub struct Region {
    row_count: u32,
    column_count: u32,
    requirements: HashMap<PresentId, usize>,
}

impl Region {
    #[inline]
    #[must_use]
    pub fn row_count(&self) -> u32 {
        self.row_count
    }

    #[inline]
    #[must_use]
    pub fn column_count(&self) -> u32 {
        self.column_count
    }

    #[inline]
    #[must_use]
    pub fn requirement_count(&self, id: PresentId) -> usize {
        self.requirements.get(&id).map_or(0, |id| *id)
    }

    pub fn parse(value: &str) -> Result<Option<(&str, Region)>> {
        if value.is_empty() {
            return Ok(None);
        }
        let mut value = value;
        let newline_index;
        let line;
        if let Some(index) = value.find('\n') {
            newline_index = index;
            line = &value[..newline_index];
            value = &value[index + 1..];
        } else {
            newline_index = value.len();
            line = &value[..newline_index];
            value = &value[value.len()..];
        }
        let line = line.trim_ascii_end();
        let Some((dimensions, requirements)) = line.split_once(':') else {
            return Err(AdventError::new("The region was malformed"));
        };
        let Some((row_count, column_count)) = Self::parse_dimensions(dimensions) else {
            return Err(AdventError::new("The region dimensions were malformed"));
        };
        let requirements_joined = requirements.trim_ascii_start();
        let requirements_split = requirements_joined.split(' ');
        let mut requirements = HashMap::new();
        for (id, count) in requirements_split.enumerate() {
            let Ok(count) = count.parse::<usize>() else {
                return Err(AdventError::new("Encountered an invalid requirement count"));
            };
            let Ok(id) = u32::try_from(id) else {
                return Err(AdventError::new(
                    "Encountered an abnormally large present ID",
                ));
            };
            requirements.insert(id, count);
        }
        let region = Region {
            row_count,
            column_count,
            requirements,
        };
        Ok(Some((value, region)))
    }

    fn parse_dimensions(value: &str) -> Option<(u32, u32)> {
        let (x, y) = value.split_once('x')?;
        let x = x.parse::<u32>().ok()?;
        let y = y.parse::<u32>().ok()?;
        Some((x, y))
    }
}

#[cfg(test)]
mod tests {
    use crate::region::Region;

    #[test]
    fn test_parse() {
        let line = "12x5: 1 0 1 0 3 2\n";
        let (left_overs, region) = Region::parse(line).unwrap().unwrap();
        assert!(left_overs.is_empty());
        assert_eq!(12, region.row_count());
        assert_eq!(5, region.column_count());
        assert_eq!(1, region.requirement_count(0));
        assert_eq!(0, region.requirement_count(1));
        assert_eq!(1, region.requirement_count(2));
        assert_eq!(0, region.requirement_count(3));
        assert_eq!(3, region.requirement_count(4));
        assert_eq!(2, region.requirement_count(5));
    }
}
