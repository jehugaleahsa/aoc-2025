mod connection;

use crate::connection::Connection;
use shared::{AdventError, Result};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

fn main() -> Result<()> {
    let path = Path::new("day11/resources/input.txt");
    run_part1(path)?;
    //run_part2(path)?;
    Ok(())
}

fn run_part1(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let connections = parse_connections(reader)?;
    let count = count_paths(&connections, "you", "out");
    println!("Part 1 - Total paths: {count}");
    Ok(())
}

#[allow(dead_code)]
fn run_part2(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let connections = parse_connections(reader)?;
    let lookup = build_connection_lookup(&connections);
    let mut requirements = HashSet::new();
    requirements.insert(find_key(&lookup, "dac"));
    requirements.insert(find_key(&lookup, "fft"));
    let count = count_paths_with_requirements(&lookup, "svr", "out", &requirements);
    println!("Part 2 - Total paths: {count}");
    Ok(())
}

fn parse_connections<R: BufRead>(reader: R) -> Result<Vec<Connection>> {
    let mut all_connections = Vec::new();
    for line in reader.lines() {
        let Ok(line) = line else {
            return Err(AdventError::new("Could not read the next input line"));
        };
        let Some(connections) = Connection::parse(&line) else {
            return Err(AdventError::new("Could not parse the next input line"));
        };
        all_connections.extend(connections);
    }
    Ok(all_connections)
}

fn count_paths(connections: &[Connection], start: &str, stop: &str) -> u64 {
    let lookup = build_connection_lookup(connections);
    let start = find_key(&lookup, start);
    let stop = find_key(&lookup, stop);
    let mut counts = HashMap::new();
    count_possibilities(&lookup, &start, &stop, &mut counts)
}

fn count_possibilities(
    lookup: &HashMap<Rc<String>, HashSet<Rc<String>>>,
    start: &Rc<String>,
    stop: &Rc<String>,
    counts: &mut HashMap<Rc<String>, u64>,
) -> u64 {
    if start == stop {
        counts.insert(Rc::clone(start), 1);
        return 1;
    }
    if let Some(count) = counts.get(start) {
        return *count;
    }
    let mut count = 0;
    if let Some(outputs) = lookup.get(start) {
        for output in outputs {
            count += count_possibilities(lookup, output, stop, counts);
        }
    }
    counts.insert(Rc::clone(start), count);
    count
}

fn count_paths_with_requirements(
    lookup: &HashMap<Rc<String>, HashSet<Rc<String>>>,
    start: &str,
    stop: &str,
    requirements: &HashSet<Rc<String>>,
) -> u64 {
    if start == stop {
        return 0; // We didn't visit the required machines
    }
    let start = find_key(lookup, start);
    let stop = find_key(lookup, stop);
    let mut counts = HashMap::new();
    count_possibilities(lookup, &start, &stop, &mut counts);
    let mut with_requirements = HashSet::new();
    find_machines_with_requirements(lookup, &start, &stop, requirements, &mut with_requirements);
    let mut cache = HashMap::new();
    let mut count = 0;
    if let Some(outputs) = lookup.get(&start) {
        for output in outputs {
            let mut remaining = requirements.clone();
            remaining.remove(&start);
            let mut path = Vec::new();
            //path.push(Rc::clone(&start));
            let mut visited = HashSet::new();
            visited.insert(Rc::clone(&start));
            count += count_possibilities_with_requirements(
                lookup,
                &mut cache,
                &counts,
                &mut remaining,
                output,
                &stop,
                &mut path,
                &mut visited,
                &with_requirements,
            );
        }
    }
    count
}

fn find_machines_with_requirements(
    lookup: &HashMap<Rc<String>, HashSet<Rc<String>>>,
    start: &Rc<String>,
    stop: &Rc<String>,
    requirements: &HashSet<Rc<String>>,
    with_requirements: &mut HashSet<Rc<String>>,
) -> bool {
    if with_requirements.contains(start) {
        return true;
    }
    let mut result = false;
    if requirements.contains(start) {
        result = true;
    }
    if start != stop
        && let Some(children) = lookup.get(start)
    {
        for child in children {
            if find_machines_with_requirements(lookup, child, stop, requirements, with_requirements)
            {
                result = true;
            }
        }
    }
    if result {
        with_requirements.insert(Rc::clone(start));
    }
    result
}

fn count_possibilities_with_requirements(
    lookup: &HashMap<Rc<String>, HashSet<Rc<String>>>,
    requirements_cache: &mut HashMap<Rc<String>, HashMap<Rc<String>, HashSet<Rc<String>>>>,
    counts: &HashMap<Rc<String>, u64>,
    remaining: &mut HashSet<Rc<String>>,
    start: &Rc<String>,
    stop: &Rc<String>,
    path: &mut Vec<Rc<String>>,
    visited: &mut HashSet<Rc<String>>,
    with_requirements: &HashSet<Rc<String>>,
) -> u64 {
    //path.push(Rc::clone(start));
    let removed = remaining.remove(start);
    if start == stop {
        if removed {
            remaining.insert(Rc::clone(start));
        }
        return u64::from(remaining.is_empty());
    }
    if remaining.is_empty()
        && let Some(count) = counts.get(start)
    {
        if removed {
            remaining.insert(Rc::clone(start));
        }
        return *count;
    }
    if !remaining.is_empty() && !with_requirements.contains(start) {
        if removed {
            remaining.insert(Rc::clone(start));
        }
        return 0; // No point in searching further down... no children have any requirements.
    }
    if let Some(parent_cache) = requirements_cache.get(start) {
        // We avoid looking at the rest of the machines by keeping
        // track of which requirements are met later on for a particular machine.
        // If the only missing requirements are met later on, we can just bubble
        // up the counts.
        if let Some(children) = lookup.get(start) {
            let mut happy_children: HashSet<Rc<String>> = children.iter().cloned().collect();
            for remaining in remaining.iter() {
                if let Some(children) = parent_cache.get(remaining) {
                    let children: HashSet<Rc<String>> = children.iter().cloned().collect();
                    happy_children = happy_children.intersection(&children).cloned().collect();
                    if happy_children.is_empty() {
                        return 0;
                    }
                } else {
                    // One of the remaining items isn't satisfied by any of the children.
                    return 0;
                }
            }
            return happy_children.len() as u64;
        }
        return 0;
    }
    if !visited.insert(Rc::clone(start)) {
        return 0; // Cyclic!
    }
    let mut count = 0;
    if let Some(outputs) = lookup.get(start) {
        for output in outputs {
            let child_count = count_possibilities_with_requirements(
                lookup,
                requirements_cache,
                counts,
                remaining,
                output,
                stop,
                path,
                visited,
                with_requirements,
            );
            count += child_count;
            //path.pop();

            // If our child or one of its children reached the destination, we want to track
            // which requirements were missing at this point. Since our children found the
            // destination, then the remaining requirements must have been met on their path,
            // and thus were found.
            if child_count > 0 {
                let entry = requirements_cache.entry(Rc::clone(start));
                match entry {
                    Vacant(entry) => {
                        let mut child_lookup = HashMap::new();
                        register_child_count(&mut child_lookup, remaining, output);
                        entry.insert(child_lookup);
                    }
                    Occupied(mut entry) => {
                        let child_lookup = entry.get_mut();
                        register_child_count(child_lookup, remaining, output);
                    }
                }
            }
        }
    }
    if removed {
        remaining.insert(Rc::clone(start));
    }
    visited.remove(start);
    count
}

fn register_child_count(
    child_lookup: &mut HashMap<Rc<String>, HashSet<Rc<String>>>,
    remaining: &HashSet<Rc<String>>,
    output: &Rc<String>,
) {
    for remaining in remaining {
        let entry = child_lookup.entry(Rc::clone(remaining));
        match entry {
            Vacant(entry) => {
                let mut children = HashSet::new();
                children.insert(Rc::clone(output));
                entry.insert(children);
            }
            Occupied(mut entry) => {
                let children = entry.get_mut();
                children.insert(Rc::clone(output));
            }
        }
    }
}

fn build_connection_lookup(connections: &[Connection]) -> HashMap<Rc<String>, HashSet<Rc<String>>> {
    let mut lookup = HashMap::new();
    for connection in connections {
        let Connection { input, output } = connection;
        let input = Rc::new(input.clone());
        if let Vacant(entry) = lookup.entry(input) {
            entry.insert(HashSet::new());
        }
        let output = Rc::new(output.clone());
        if let Vacant(entry) = lookup.entry(output) {
            entry.insert(HashSet::new());
        }
    }
    for connection in connections {
        let output = lookup.get_key_value(&connection.output).unwrap().0;
        let output = Rc::clone(output);
        let outputs = lookup.get_mut(&connection.input).unwrap();
        outputs.insert(output);
    }
    lookup
}

fn find_key(
    lookup: &HashMap<Rc<String>, HashSet<Rc<String>>>,
    key: impl Into<String>,
) -> Rc<String> {
    let key = Rc::new(key.into());
    let key = lookup.get_key_value(&key).unwrap().0;
    Rc::clone(key)
}

#[cfg(test)]
mod tests {
    use crate::{
        build_connection_lookup, count_paths, count_paths_with_requirements, find_key,
        parse_connections,
    };
    use std::collections::HashSet;
    use std::io::Cursor;

    #[test]
    fn test_part1_example() {
        let cursor = create_part1_cursor();
        let connections = parse_connections(cursor).unwrap();
        let path_count = count_paths(&connections, "you", "out");
        assert_eq!(5, path_count);
    }

    fn create_part1_cursor() -> Cursor<String> {
        let lines = [
            "aaa: you hhh",
            "you: bbb ccc",
            "bbb: ddd eee",
            "ccc: ddd eee fff",
            "ddd: ggg",
            "eee: out",
            "fff: out",
            "ggg: out",
            "hhh: ccc fff iii",
            "iii: out",
        ];
        let joined = lines.join("\n");
        Cursor::new(joined)
    }

    #[test]
    fn test_part2_example_without_requirements() {
        let cursor = create_part2_cursor();
        let connections = parse_connections(cursor).unwrap();
        let path_count = count_paths(&connections, "svr", "out");
        assert_eq!(8, path_count);
    }

    #[test]
    fn test_part2_example() {
        let cursor = create_part2_cursor();
        let connections = parse_connections(cursor).unwrap();
        let lookup = build_connection_lookup(&connections);
        let mut requirements = HashSet::new();
        requirements.insert(find_key(&lookup, "dac"));
        requirements.insert(find_key(&lookup, "fft"));
        let path_count = count_paths_with_requirements(&lookup, "svr", "out", &requirements);
        assert_eq!(2, path_count);
    }

    fn create_part2_cursor() -> Cursor<String> {
        let lines = [
            "svr: aaa bbb",
            "aaa: fft",
            "fft: ccc",
            "bbb: tty",
            "tty: ccc",
            "ccc: ddd eee",
            "ddd: hub",
            "hub: fff",
            "eee: dac",
            "dac: fff",
            "fff: ggg hhh",
            "ggg: out",
            "hhh: out",
        ];
        let joined = lines.join("\n");
        Cursor::new(joined)
    }
}
