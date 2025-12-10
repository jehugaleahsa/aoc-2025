mod marked_tiles;
mod square;
mod tile;

use crate::marked_tiles::MarkedTiles;
use crate::square::Square;
use crate::tile::Tile;
use rayon::prelude::*;
use shared::{AdventError, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

fn main() -> Result<()> {
    let path = Path::new("day09/resources/input.txt");
    run_part1(path)?;
    run_part2_rayon(path)?;
    Ok(())
}

fn run_part1(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let tiles = parse_tiles(reader)?;
    let mut squares = create_squares(&tiles, false);
    sort_squares_by_area_desc(&mut squares);
    if let Some(square) = squares.first() {
        println!("Part 1 - Largest Square: {}", square.area());
    }
    Ok(())
}

#[allow(dead_code)] // Part 2 incomplete
fn run_part2(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let tiles = parse_tiles(reader)?;
    if let Some(largest_square) = find_largest_square(&tiles, false) {
        let area = largest_square.area();
        println!("Part 2 - Largest Area: {area}");
    }
    Ok(())
}

fn find_largest_square(tiles: &Vec<Tile>, bounded: bool) -> Option<Square> {
    let marked = mark_valid_tiles(&tiles)?;
    let mut squares = create_squares(&tiles, bounded);
    let square_count = squares.len();
    sort_squares_by_area_desc(&mut squares);
    squares
        .into_iter()
        .enumerate()
        .filter(|(i, s)| {
            println!("Attempting {} of {square_count} - area {}", i, s.area());
            all_valid_tiles(s, &marked)
        })
        .map(|p| p.1)
        .next()
}

fn run_part2_rayon(path: &Path) -> Result<()> {
    let Ok(file) = File::open(path) else {
        return Err(AdventError::new("Could not open the input file"));
    };
    let reader = BufReader::new(file);
    let tiles = parse_tiles(reader)?;
    if let Some(largest_square) = find_largest_square_rayon(&tiles, false) {
        let area = largest_square.area();
        println!("Part 2 - Largest Area: {area}");
    }
    Ok(())
}

fn find_largest_square_rayon(tiles: &Vec<Tile>, bounded: bool) -> Option<Square> {
    let marked = mark_valid_tiles(&tiles)?;
    let squares = create_squares(&tiles, bounded);
    //let squares: Vec<Square> = squares
    //.into_iter()
    //.filter(|s| {
    //    let area = s.area();
    //    area < 2310190800 && area > 1286398100
    //})
    //.collect();
    let square_count = squares.len();
    let mut valid_squares = Vec::new();
    let done = AtomicUsize::new(0);
    let largest = AtomicU64::new(0);
    valid_squares.par_extend(
        squares
            .into_par_iter()
            .enumerate()
            .filter(|(i, s)| {
                let old = done.fetch_add(1, Ordering::Relaxed);
                let percent_complete = old as f32 / square_count as f32;
                let area = s.area();
                println!(
                    "Attempting {} of {square_count} - area {} - {}",
                    i, area, percent_complete
                );
                let result = all_valid_tiles(s, &marked);
                if result {
                    largest.fetch_max(area, Ordering::Relaxed);
                    let new_largest = largest.load(Ordering::SeqCst);
                    println!("Max found: {new_largest}");
                }
                result
            })
            .map(|p| p.1),
    );
    sort_squares_by_area_desc(&mut valid_squares);
    valid_squares.first().cloned()
}

fn all_valid_tiles(square: &Square, marked_tiles: &MarkedTiles) -> bool {
    let min_x = square.first.x.min(square.second.x);
    let min_y = square.first.y.min(square.second.y);
    let max_x = square.first.x.max(square.second.x);
    let max_y = square.first.y.max(square.second.y);
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !marked_tiles.is_set(x, y) {
                return false;
            }
        }
    }
    true
}

fn mark_valid_tiles(tiles: &Vec<Tile>) -> Option<MarkedTiles> {
    let Some(first) = tiles.first() else {
        return None;
    };
    let min_x = tiles
        .iter()
        .map(|t| t.x)
        .min()
        .expect("At least one tile expected");
    let max_x = tiles
        .iter()
        .map(|t| t.x)
        .max()
        .expect("At least one tile expected");
    let min_y = tiles
        .iter()
        .map(|t| t.y)
        .min()
        .expect("At least one tile expected");
    let max_y = tiles
        .iter()
        .map(|t| t.y)
        .max()
        .expect("At least one tile expected");
    let mut marked_tiles = MarkedTiles::new(min_x, max_x, min_y, max_y);

    // Draw the outline
    let nexts = tiles.iter().skip(1).chain(vec![first].into_iter());
    let pairs = tiles.iter().zip(nexts);
    for (first, second) in pairs {
        if first.x == second.x {
            // The two tiles are on the same row
            let min_y = first.y.min(second.y);
            let max_y = first.y.max(second.y);
            for y in min_y..=max_y {
                marked_tiles.set(first.x, y);
            }
        } else if first.y == second.y {
            // The two tiles are on the same column
            let min_x = first.x.min(second.x);
            let max_x = first.x.max(second.x);
            for x in min_x..=max_x {
                marked_tiles.set(x, first.y);
            }
        }
    }

    // Find the top-left corner and the bottom-right corner and mark all spots between
    // red or green tiles.
    let mut y_count = 0u32;
    for y in min_y..=max_y {
        let mut start_x = None;
        let mut end_x = None;
        y_count += 1;
        if y_count % 1_000 == 0 {
            println!("{y}");
        }
        for x in min_x..=max_x {
            if marked_tiles.is_set(x, y) {
                // If we see an occupied tile, we're either at the start of a valid range,
                // or we're in the middle somewhere. If we previously encountered a start,
                // then we
                if let Some(first_x) = start_x
                    && let Some(last_x) = end_x
                {
                    for x in (first_x + 1)..=last_x {
                        marked_tiles.set(x, y);
                    }
                    start_x = None;
                    end_x = None;
                } else {
                    start_x = Some(x);
                }
            } else if start_x.is_some() {
                // We keep track of the last blank tile we encountered.
                end_x = Some(x);
            }
        }
    }

    Some(marked_tiles)
}

fn create_squares(tiles: &Vec<Tile>, bounded: bool) -> Vec<Square> {
    let mut squares = Vec::new();
    for outer_index in 0..tiles.len() {
        let previous_index = if outer_index == 0 {
            tiles.len() - 1
        } else {
            outer_index - 1
        };
        let next_index = if outer_index + 1 == tiles.len() {
            0
        } else {
            outer_index + 1
        };
        let outer_tile = tiles[outer_index];
        let previous_tile = tiles[previous_index];
        let next_tile = tiles[next_index];
        if is_top_left_corner(&outer_tile, &previous_tile, &next_tile) {
            for inner_index in 0..tiles.len() {
                if inner_index != outer_index {
                    let inner_tile = tiles[inner_index];
                    // Since we're starting in the top-left corner, we only want to look
                    // at other red tiles that are to our right and below us.
                    if inner_tile.x >= outer_tile.x
                        && inner_tile.y >= outer_tile.y
                        && (!bounded || inner_tile.x >= next_tile.x)
                    {
                        let square = Square::new(outer_tile, inner_tile);
                        squares.push(square);
                    }
                }
            }
        } else if is_bottom_left_corner(&outer_tile, &previous_tile, &next_tile) {
            for inner_index in (outer_index + 1)..tiles.len() {
                if inner_index != outer_index {
                    let inner_tile = tiles[inner_index];
                    // Since we're the bottom-left corner, we only want to look
                    // at other red tiles that are to our right and above us.
                    if inner_tile.x >= outer_tile.x
                        && inner_tile.y <= outer_tile.y
                        && (!bounded || inner_tile.x <= previous_tile.x)
                    {
                        let square = Square::new(outer_tile, inner_tile);
                        squares.push(square);
                    }
                }
            }
        }
    }
    squares
}

fn is_top_left_corner(current: &Tile, previous: &Tile, next: &Tile) -> bool {
    current.y == next.y && previous.x == current.x && current.x <= next.x && current.y <= previous.y
}

fn is_bottom_left_corner(current: &Tile, previous: &Tile, next: &Tile) -> bool {
    next.x == current.x && current.y == previous.y && current.x <= previous.x && current.y >= next.y
}

fn sort_squares_by_area_desc(squares: &mut Vec<Square>) {
    squares.sort_by(|a, b| a.area().partial_cmp(&b.area()).unwrap().reverse())
}

fn parse_tiles<R: BufRead>(reader: R) -> Result<Vec<Tile>> {
    let mut tiles = Vec::new();
    for line in reader.lines() {
        let Ok(line) = line else {
            return Err(AdventError::new("Could not read the next input line"));
        };
        let Some((x, y)) = line.split_once(',') else {
            return Err(AdventError::new("Encountered an invalid tile coordinate"));
        };
        let Ok(x) = x.parse::<u32>() else {
            return Err(AdventError::new("Encountered an invalid tile X coordinate"));
        };
        let Ok(y) = y.parse::<u32>() else {
            return Err(AdventError::new("Encountered an invalid tile Y coordinate"));
        };
        let tile = Tile::from_x_y(x, y);
        tiles.push(tile);
    }
    Ok(tiles)
}

#[cfg(test)]
mod tests {
    use crate::tile::Tile;
    use crate::{
        create_squares, find_largest_square, is_bottom_left_corner, is_top_left_corner,
        parse_tiles, sort_squares_by_area_desc,
    };
    use std::io::Cursor;

    #[test]
    fn test_part1_example() {
        let cursor = create_cursor();
        let tiles = parse_tiles(cursor).unwrap();
        let mut squares = create_squares(&tiles, false);
        sort_squares_by_area_desc(&mut squares);
        let largest_square = squares.first().unwrap();
        let largest_area = largest_square.area();
        assert_eq!(50, largest_area);
    }

    #[test]
    fn test_part2_example() {
        let cursor = create_cursor();
        let tiles = parse_tiles(cursor).unwrap();
        let largest_square = find_largest_square(&tiles, true).unwrap();
        let area = largest_square.area();
        assert_eq!(24, area);
    }

    #[test]
    fn test_is_top_left_corner_positive() {
        let current = Tile::from_x_y(0, 0);
        let next = Tile::from_x_y(10, 0);
        let previous = Tile::from_x_y(0, 10);
        let result = is_top_left_corner(&current, &previous, &next);
        assert!(result);
    }

    #[test]
    fn test_is_top_left_corner_negative() {
        let previous = Tile::from_x_y(0, 0);
        let current = Tile::from_x_y(10, 0);
        let next = Tile::from_x_y(10, 10);
        let result = is_top_left_corner(&current, &previous, &next);
        assert!(!result);
    }

    #[test]
    fn test_is_bottom_left_corner_positive() {
        let current = Tile::from_x_y(0, 10);
        let next = Tile::from_x_y(0, 0);
        let previous = Tile::from_x_y(10, 10);
        let result = is_bottom_left_corner(&current, &previous, &next);
        assert!(result);
    }

    #[test]
    fn test_is_bottom_left_corner_negative() {
        let next = Tile::from_x_y(0, 10);
        let current = Tile::from_x_y(10, 10);
        let previous = Tile::from_x_y(10, 0);
        let result = is_bottom_left_corner(&current, &previous, &next);
        assert!(!result);
    }

    fn create_cursor() -> Cursor<String> {
        let lines = ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];
        let joined = lines.join("\n");
        Cursor::new(joined)
    }
}
