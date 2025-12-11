use crate::circuit::Circuit;
use crate::connection::Connection;
use crate::junction::Junction;
use shared::{AdventError, Result};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry::Vacant;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;

mod circuit;
mod connection;
mod junction;

fn main() -> Result<()> {
    let path = Path::new("day08/resources/input.txt");
    run_part1(path)?;
    run_part2(path)?;
    Ok(())
}

fn run_part1(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let junctions = parse_junctions(reader)?;
    let mut connections = create_combinations(&junctions);
    sort_combinations(&mut connections);
    let mut circuits = connect_combinations(&connections, 1_000, false, junctions.len());
    sort_circuits_by_total_connections(&mut circuits);
    let product = circuits
        .iter()
        .take(3)
        .map(|c| c.borrow().len())
        .product::<usize>();
    println!("Part 1 - Product: {product}");
    Ok(())
}

fn run_part2(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let junctions = parse_junctions(reader)?;
    let mut connections = create_combinations(&junctions);
    sort_combinations(&mut connections);
    connect_combinations(&connections, connections.len(), true, junctions.len());
    Ok(())
}

fn parse_junctions<R: BufRead>(reader: R) -> Result<Vec<Junction>> {
    let mut junctions = Vec::new();
    for line in reader.lines() {
        let Ok(line) = line else {
            return Err(AdventError::new("Could not read the next input line"));
        };
        let line = line.trim_end_matches('\r');
        let parts: Vec<&str> = line.split(',').collect();
        let [x, y, z] = *parts.as_slice() else {
            return Err(AdventError::new(
                "Encountered invalid junction box coordinates",
            ));
        };
        let Ok(x) = x.parse() else {
            return Err(AdventError::new(
                "Encountered an invalid junction box X coordinate",
            ));
        };
        let Ok(y) = y.parse() else {
            return Err(AdventError::new(
                "Encountered an invalid junction box Y coordinate",
            ));
        };
        let Ok(z) = z.parse() else {
            return Err(AdventError::new(
                "Encountered an invalid junction box Z coordinate",
            ));
        };
        let junction = Junction::from_x_y_z(x, y, z);
        junctions.push(junction);
    }
    Ok(junctions)
}

fn create_combinations(junctions: &[Junction]) -> Vec<Connection> {
    let mut connections = Vec::new();
    for outer_index in 0..junctions.len() {
        let first = junctions[outer_index];
        for inner_index in (outer_index + 1)..junctions.len() {
            let second = junctions[inner_index];
            let connection = Connection { first, second };
            connections.push(connection);
        }
    }
    connections
}

fn sort_combinations(connection: &mut [Connection]) {
    connection.sort_by(|a, b| {
        let a_distance = a.distance();
        let b_distance = b.distance();
        a_distance.partial_cmp(&b_distance).unwrap()
    });
}

fn connect_combinations(
    combinations: &[Connection],
    max_connections: usize,
    part2: bool,
    total_junctions: usize,
) -> Vec<Rc<RefCell<Circuit>>> {
    let mut unique_circuits: HashMap<*const Circuit, Rc<RefCell<Circuit>>> = HashMap::new();
    let mut junction_circuits: HashMap<Junction, Rc<RefCell<Circuit>>> = HashMap::new();
    for combination in combinations.iter().take(max_connections) {
        let first_circuit = junction_circuits.get(&combination.first);
        let second_circuit = junction_circuits.get(&combination.second);
        match (first_circuit, second_circuit) {
            (None, None) => {
                let mut new_circuit = Circuit::new();
                new_circuit.add(combination.first);
                new_circuit.add(combination.second);
                let new_circuit = Rc::new(RefCell::new(new_circuit));
                junction_circuits.insert(combination.first, Rc::clone(&new_circuit));
                junction_circuits.insert(combination.second, Rc::clone(&new_circuit));
                if let Vacant(entry) = unique_circuits.entry(new_circuit.as_ptr()) {
                    entry.insert(new_circuit);
                }
            }
            (Some(first_circuit), None) => {
                first_circuit.borrow_mut().add(combination.second);
                junction_circuits.insert(combination.second, Rc::clone(first_circuit));
            }
            (None, Some(second_circuit)) => {
                second_circuit.borrow_mut().add(combination.first);
                junction_circuits.insert(combination.first, Rc::clone(second_circuit));
            }
            (Some(first_circuit), Some(second_circuit)) => {
                if first_circuit.as_ptr() != second_circuit.as_ptr() {
                    unique_circuits.remove(&std::ptr::from_ref::<Circuit>(
                        first_circuit.borrow().deref(),
                    ));
                    unique_circuits.remove(&std::ptr::from_ref::<Circuit>(
                        second_circuit.borrow().deref(),
                    ));

                    let new_circuit = first_circuit
                        .borrow()
                        .merge(second_circuit.borrow().deref());
                    let new_circuit = Rc::new(RefCell::new(new_circuit));
                    for junction in new_circuit.borrow().junctions() {
                        junction_circuits.insert(*junction, Rc::clone(&new_circuit));
                    }
                    unique_circuits.insert(new_circuit.as_ptr(), new_circuit);
                }
            }
        }
        if part2
            && unique_circuits.len() == 1
            && let Some(circuit) = unique_circuits.values().next()
            && circuit.borrow().len() == total_junctions
        {
            let distance = combination.first.x * combination.second.x;
            println!("Part 2 - Distance: {distance}");
            break;
        }
    }
    unique_circuits.into_values().collect()
}

fn sort_circuits_by_total_connections(circuits: &mut [Rc<RefCell<Circuit>>]) {
    circuits.sort_by(|a, b| a.borrow().len().cmp(&b.borrow().len()).reverse());
}

#[cfg(test)]
mod tests {
    use crate::junction::Junction;
    use crate::{
        connect_combinations, create_combinations, parse_junctions,
        sort_circuits_by_total_connections, sort_combinations,
    };
    use std::io::Cursor;

    #[test]
    fn test_part1_example() {
        let cursor = create_cursor();
        let junctions = parse_junctions(cursor).unwrap();
        assert_eq!(20, junctions.len());
        let mut combinations = create_combinations(&junctions);
        sort_combinations(&mut combinations);
        let closest = combinations.first().unwrap();
        assert_eq!(Junction::from_x_y_z(162, 817, 812), closest.first);
        assert_eq!(Junction::from_x_y_z(425, 690, 689), closest.second);

        let mut circuits = connect_combinations(&combinations, 10, false, junctions.len());
        sort_circuits_by_total_connections(&mut circuits);
        let [a, b, c, ..] = circuits.as_slice() else {
            panic!("There should have been at least 3 circuits");
        };
        assert_eq!(5, a.borrow().len());
        assert_eq!(4, b.borrow().len());
        assert_eq!(2, c.borrow().len());
    }

    fn create_cursor() -> Cursor<String> {
        const EXAMPLE: [&str; 20] = [
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ];
        let joined = EXAMPLE.join("\n");
        Cursor::new(joined)
    }
}
