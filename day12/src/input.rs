use crate::present::Present;
use crate::region::Region;
use shared::{AdventError, Result};

#[derive(Debug)]
pub struct Input {
    pub presents: Vec<Present>,
    pub regions: Vec<Region>,
}

impl Input {
    #[inline]
    #[must_use]
    pub fn presents(&self) -> &[Present] {
        &self.presents
    }

    #[inline]
    #[must_use]
    pub fn regions(&self) -> &[Region] {
        &self.regions
    }

    fn parse(value: &str) -> Result<Input> {
        let mut value = value;
        let mut presents = Vec::new();
        // We continue parsing presents until the parse operation fails.
        // We initially assume the failure is due to switching over to regions.
        while let Ok(result) = Present::parse(value) {
            let Some((slice, present)) = result else {
                break;
            };
            presents.push(present);
            value = slice;
        }
        let mut regions = Vec::new();
        while let Some((slice, region)) = Region::parse(value)? {
            regions.push(region);
            value = slice;
        }
        if !value.is_empty() {
            return Err(AdventError::new(
                "Encountered trailing content in the input file",
            ));
        }
        let input = Input { presents, regions };
        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::input::Input;

    #[test]
    fn test_parse() {
        let lines = [
            "0:",
            "###",
            "##.",
            "##.",
            "",
            "1:",
            "###",
            "##.",
            ".##",
            "",
            "2:",
            ".##",
            "###",
            "##.",
            "",
            "3:",
            "##.",
            "###",
            "##.",
            "",
            "4:",
            "###",
            "#..",
            "###",
            "",
            "5:",
            "###",
            ".#.",
            "###",
            "",
            "4x4: 0 0 0 0 2 0",
            "12x5: 1 0 1 0 2 2",
            "12x5: 1 0 1 0 3 2",
        ];
        let joined = lines.join("\n");
        let input = Input::parse(&joined).unwrap();
        assert_eq!(6, input.presents().len());
        assert_eq!(3, input.regions().len());
    }
}
