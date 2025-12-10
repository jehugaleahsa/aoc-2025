use std::cmp::Ordering;

#[derive(Debug)]
pub struct Schematic {
    required_indicator_lights: Vec<bool>,
    button_activations: Vec<Vec<bool>>,
    required_joltages: Vec<u32>,
}

impl Schematic {
    #[inline]
    #[must_use]
    pub fn indicator_light_count(&self) -> usize {
        self.required_indicator_lights.len()
    }

    #[inline]
    #[must_use]
    pub fn joltage_count(&self) -> usize {
        self.required_joltages.len()
    }

    #[must_use]
    pub fn is_required_indicator_lights(&self, lights: &[bool]) -> bool {
        for index in 0..self.required_indicator_lights.len() {
            if self.required_indicator_lights[index] != lights[index] {
                return false;
            }
        }
        true
    }

    #[inline]
    #[must_use]
    pub fn button_count(&self) -> usize {
        self.button_activations.len()
    }

    pub fn press_button_for_lights(&self, button_index: usize, lights: &mut [bool]) {
        let button = &self.button_activations[button_index];
        for index in 0..button.len() {
            if button[index] {
                lights[index] = !lights[index];
            }
        }
    }

    pub fn press_button_for_joltages(&self, button_index: usize, joltages: &mut [u32]) -> Ordering {
        let button = &self.button_activations[button_index];
        let required_joltages = &self.required_joltages;
        let mut ordering = Ordering::Equal;
        for index in 0..button.len() {
            if button[index] {
                joltages[index] += 1;
                match joltages[index].cmp(&required_joltages[index]) {
                    Ordering::Less => ordering = Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => {}
                }
            }
        }
        ordering
    }

    #[inline]
    #[must_use]
    pub fn required_joltages(&self) -> &[u32] {
        &self.required_joltages
    }

    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        let mut slice = value.trim_ascii_start();
        if slice.is_empty() || !slice.starts_with('[') {
            return None;
        }
        slice = &slice[1..];
        let indicator_end = slice.find(']')?;
        let required_indicator_lights = Self::parse_indicator_lights(&slice[..indicator_end])?;
        let indicator_count = required_indicator_lights.len();
        slice = &slice[indicator_end + 1..];
        slice = slice.trim_ascii_start();
        let button_end = slice.find('{')?;
        let button_activations =
            Self::parse_button_activations(&slice[..button_end], indicator_count)?;
        slice = &slice[button_end + 1..];
        let joltage_end = slice.find('}')?;
        let required_joltages = Self::parse_joltages(&slice[..joltage_end])?;
        slice = &slice[joltage_end + 1..];
        slice = slice.trim_ascii_start();
        if !slice.is_empty() {
            return None;
        }
        let schematic = Self {
            required_indicator_lights,
            button_activations,
            required_joltages,
        };
        Some(schematic)
    }

    fn parse_indicator_lights(value: &str) -> Option<Vec<bool>> {
        let mut lights = Vec::with_capacity(value.len());
        for next in value.chars() {
            match next {
                '.' => lights.push(false),
                '#' => lights.push(true),
                _ => return None,
            }
        }
        Some(lights)
    }

    fn parse_button_activations(value: &str, button_count: usize) -> Option<Vec<Vec<bool>>> {
        let mut slice = value.trim_ascii_start();
        let mut buttons = Vec::new();
        while !slice.is_empty() {
            if !slice.starts_with('(') {
                return None;
            }
            slice = &slice[1..];
            let button_end = slice.find(')')?;
            let button = Self::parse_button(&slice[..button_end], button_count)?;
            buttons.push(button);
            slice = &slice[button_end + 1..];
            slice = slice.trim_ascii_start();
        }
        Some(buttons)
    }

    fn parse_button(value: &str, button_count: usize) -> Option<Vec<bool>> {
        let mut activations = vec![false; button_count];
        let indexes = Self::parse_comma_separated_numbers(value)?;
        for index in indexes {
            if index < button_count {
                activations[index] = true;
            } else {
                return None;
            }
        }
        Some(activations)
    }

    fn parse_joltages(value: &str) -> Option<Vec<u32>> {
        let joltages: Vec<u32> = Self::parse_comma_separated_numbers(value)?
            .into_iter()
            .map(|i| i as u32)
            .collect();
        Some(joltages)
    }

    fn parse_comma_separated_numbers(value: &str) -> Option<Vec<usize>> {
        let mut numbers = Vec::new();
        let mut slice = value.trim_ascii_start();
        while !slice.is_empty() {
            let comma = slice.find(',').unwrap_or(slice.len());
            let number = &slice[..comma];
            let activation = number.parse::<usize>().ok()?;
            numbers.push(activation);
            if comma == slice.len() {
                break;
            }
            slice = &slice[comma + 1..];
            slice = slice.trim_ascii_start();
        }
        Some(numbers)
    }
}

#[cfg(test)]
mod tests {
    use crate::schematic::Schematic;
    use std::fmt::Debug;

    #[test]
    fn test_parse_schematic() {
        let value = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let schematic = Schematic::parse(value).unwrap();
        assert_eq!(4, schematic.button_count());
        assert!(schematic.is_required_indicator_lights(&[false, true, true, false]));
        assert_eq!(6, schematic.button_count());
        assert_button_press(&schematic, 0, &[false, false, false, true]);
        assert_button_press(&schematic, 1, &[false, true, false, true]);
        assert_button_press(&schematic, 2, &[false, false, true, false]);
        assert_button_press(&schematic, 3, &[false, false, true, true]);
        assert_button_press(&schematic, 4, &[true, false, true, false]);
        assert_button_press(&schematic, 5, &[true, true, false, false]);
        let required_joltages = schematic.required_joltages();
        assert_array(required_joltages, &[3, 5, 4, 7]);
    }

    fn assert_button_press(schematic: &Schematic, button_index: usize, expected: &[bool]) {
        let mut lights = vec![false; schematic.button_count()];
        schematic.press_button_for_lights(button_index, &mut lights);
        assert_array(&lights, expected);
    }

    fn assert_array<T: Debug + Eq>(actual: &[T], expected: &[T]) {
        for index in 0..expected.len() {
            assert_eq!(actual[index], expected[index]);
        }
    }
}
