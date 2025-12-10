use crate::schematic::Schematic;
use shared::{AdventError, Result};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod schematic;

#[derive(Debug)]
struct LightConfiguration {
    pub lights: Vec<bool>,
    pub depth: usize,
}

#[derive(Debug)]
#[allow(dead_code)] // Part 2 incomplete
struct JoltageConfiguration {
    pub joltages: Vec<u32>,
    pub depth: usize,
    pub distance: f32,
    pub score: f32,
}

fn main() -> Result<()> {
    let path = Path::new("day10/resources/input.txt");
    run_part_1(path)?;
    //run_part_2(path)?;
    Ok(())
}

fn run_part_1(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let total = total_button_presses_for_lights(reader)?;
    println!("Part 1 - Minimal button presses for lights: {total}");

    Ok(())
}

fn total_button_presses_for_lights<R: BufRead>(reader: R) -> Result<usize> {
    let mut total = 0;
    for line in reader.lines() {
        let Ok(line) = line else {
            return Err(AdventError::new("Could not read the next input line"));
        };
        let Some(schematic) = Schematic::parse(&line) else {
            return Err(AdventError::new("Could not parse the machine schematic"));
        };
        let depth = find_minimal_button_presses_for_lights(&schematic);
        total += depth;
    }
    Ok(total)
}

fn find_minimal_button_presses_for_lights(schematic: &Schematic) -> usize {
    // We're going to try to solve this using a breadth-first algorithm. After each button press
    // we see if we've found the desired light configuration. Otherwise, we try every button
    // press off that configuration, gradually working our way deeper and deeper into the
    // tree of possibilities.
    let mut queue = VecDeque::new();
    let initial_configuration = LightConfiguration {
        lights: vec![false; schematic.indicator_light_count()],
        depth: 0,
    };
    queue.push_back(initial_configuration);
    while let Some(configuration) = queue.pop_front() {
        for button_index in 0..schematic.button_count() {
            let mut lights = configuration.lights.clone();
            schematic.press_button_for_lights(button_index, &mut lights);
            let next_depth = configuration.depth + 1;
            if schematic.is_required_indicator_lights(&lights) {
                return next_depth; // Include the current button!
            }
            let next_configuration = LightConfiguration {
                lights,
                depth: next_depth,
            };
            queue.push_back(next_configuration);
        }
    }
    unreachable!("We will never stop adding more states to the queue until we find a solution");
}

#[allow(dead_code)] // Part 2 incomplete
fn run_part_2(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let total = total_button_presses_for_joltages(reader)?;
    println!("Part 2 - Minimal button presses for joltages: {total}");

    Ok(())
}

fn total_button_presses_for_joltages<R: BufRead>(reader: R) -> Result<usize> {
    let mut total = 0;
    for line in reader.lines() {
        let Ok(line) = line else {
            return Err(AdventError::new("Could not read the next input line"));
        };
        let Some(schematic) = Schematic::parse(&line) else {
            return Err(AdventError::new("Could not parse the machine schematic"));
        };
        let Some(depth) = find_minimal_button_pressed_for_joltages_depth_first(&schematic) else {
            return Err(AdventError::new(
                "Encountered a joltage configuration without a solution!",
            ));
        };
        total += depth;
    }
    Ok(total)
}

#[allow(dead_code)] // Part 2 incomplete
fn find_minimal_button_pressed_for_joltages_breadth_first(schematic: &Schematic) -> Option<usize> {
    // We can greatly limit the number of options we try. Since only some buttons toggle
    // certain joltages, we know that those buttons must be part of button press combination.
    // We pick which buttons to press in what order more carefully. We track how far away
    // a particular combination is away from the desired joltage. We prioritize the buttons
    // that will get us closer to our desired joltage. This is essentially a "cost function".
    let mut queue = VecDeque::new();
    let initial_joltages = vec![0; schematic.joltage_count()];
    let initial_distance = compute_distance(schematic, &initial_joltages);
    let initial_configuration = JoltageConfiguration {
        joltages: initial_joltages,
        depth: 0,
        distance: initial_distance,
        score: 0.0,
    };
    queue.push_back(initial_configuration);
    while let Some(configuration) = queue.pop_front() {
        // First determine what the alternatives are.
        let next_depth = configuration.depth + 1;
        let mut alternatives = Vec::new();
        for button_index in 0..schematic.button_count() {
            let mut joltages = configuration.joltages.clone();
            let ordering = schematic.press_button_for_joltages(button_index, &mut joltages);
            match ordering {
                Ordering::Greater => continue,
                Ordering::Equal => return Some(next_depth),
                Ordering::Less => {
                    let distance = compute_distance(schematic, &joltages);
                    let score = compute_score(configuration.distance, distance, next_depth);
                    let next_configuration = JoltageConfiguration {
                        joltages,
                        depth: next_depth,
                        distance,
                        score,
                    };
                    alternatives.push(next_configuration);
                }
            }
        }
        // Now we will sort the alternatives based on which one gets us closer to our
        // end goal. We can compute this simply as the difference of each joltage. The
        // closer to the desired joltage an alternative takes us, the higher priority
        // it has.
        alternatives.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        queue.extend(alternatives);
    }
    None
}

fn find_minimal_button_pressed_for_joltages_depth_first(schematic: &Schematic) -> Option<usize> {
    // We can greatly limit the number of options we try. Since only some buttons toggle
    // certain joltages, we know that those buttons must be part of button press combination.
    // We pick which buttons to press in what order more carefully. We track how far away
    // a particular combination is away from the desired joltage. We prioritize the buttons
    // that will get us closer to our desired joltage. This is essentially a "cost function".
    let mut queue = Vec::new();
    let initial_joltages = vec![0; schematic.joltage_count()];
    let initial_distance = compute_distance(schematic, &initial_joltages);
    let initial_configuration = JoltageConfiguration {
        joltages: initial_joltages,
        depth: 0,
        distance: initial_distance,
        score: 0.0,
    };
    queue.push(initial_configuration);
    let mut running_minimum = None;
    while let Some(configuration) = queue.pop() {
        // First determine what the alternatives are.
        let next_depth = configuration.depth + 1;
        if let Some(current_minimum) = running_minimum
            && next_depth >= current_minimum
        {
            continue;
        }
        let mut alternatives = Vec::new();
        for button_index in 0..schematic.button_count() {
            let mut joltages = configuration.joltages.clone();
            let ordering = schematic.press_button_for_joltages(button_index, &mut joltages);
            match ordering {
                Ordering::Greater => continue,
                Ordering::Equal => {
                    if let Some(current_minimum) = running_minimum {
                        if current_minimum > next_depth {
                            running_minimum = Some(next_depth);
                        }
                    } else {
                        running_minimum = Some(next_depth);
                    }
                }
                Ordering::Less => {
                    let distance = compute_distance(schematic, &joltages);
                    let score = compute_score(configuration.distance, distance, next_depth);
                    let next_configuration = JoltageConfiguration {
                        joltages,
                        depth: next_depth,
                        distance,
                        score,
                    };
                    alternatives.push(next_configuration);
                }
            }
        }
        // Now we will sort the alternatives based on which one gets us closer to our
        // end goal. We can compute this simply as the difference of each joltage. The
        // closer to the desired joltage an alternative takes us, the higher priority
        // it has.
        if !alternatives.is_empty() {
            queue.extend(alternatives);
            queue.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap().reverse());
        }
    }
    running_minimum
}

fn compute_distance(schematic: &Schematic, current_joltages: &[u32]) -> f32 {
    let required_joltages = schematic.required_joltages();
    let mut total = 0;
    for index in 0..required_joltages.len() {
        let required = required_joltages[index];
        let current = current_joltages[index];
        let difference = required - current; // We are guaranteed this will be positive!
        let squared = difference * difference;
        total += squared as i32;
    }
    (total as f32).sqrt()
}

fn compute_score(old_distance: f32, new_distance: f32, depth: usize) -> f32 {
    let improvement = old_distance - new_distance;
    new_distance + improvement + depth as f32
}

#[cfg(test)]
mod tests {
    use crate::find_minimal_button_presses_for_lights;
    use crate::schematic::Schematic;

    #[test]
    fn test_part1_example1() {
        let schematic =
            Schematic::parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap();
        let depth = find_minimal_button_presses_for_lights(&schematic);
        assert_eq!(2, depth);
    }

    #[test]
    fn test_part1_example2() {
        let schematic =
            Schematic::parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
                .unwrap();
        let depth = find_minimal_button_presses_for_lights(&schematic);
        assert_eq!(3, depth);
    }

    #[test]
    fn test_part1_example3() {
        let schematic =
            Schematic::parse("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
                .unwrap();
        let depth = find_minimal_button_presses_for_lights(&schematic);
        assert_eq!(2, depth);
    }
}
