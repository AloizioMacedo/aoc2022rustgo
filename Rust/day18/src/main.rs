use std::{collections::HashSet, num::ParseIntError, sync::Mutex};

use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct CubePos(i32, i32, i32);

impl CubePos {
    fn is_adjacent(&self, other: &CubePos) -> bool {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()) == 1
    }

    fn get_adjacents(&self) -> [CubePos; 6] {
        [
            CubePos(self.0 + 1, self.1, self.2),
            CubePos(self.0 - 1, self.1, self.2),
            CubePos(self.0, self.1 + 1, self.2),
            CubePos(self.0, self.1 - 1, self.2),
            CubePos(self.0, self.1, self.2 + 1),
            CubePos(self.0, self.1, self.2 - 1),
        ]
    }
}

fn get_cubes(contents: &str) -> Result<Vec<CubePos>, ParseError> {
    contents.lines().map(parse_line).collect()
}

#[derive(Error, Debug)]
#[error("Parse error")]
enum ParseError {
    ParseIntError(#[from] ParseIntError),
    SyntaxError(String),
}

fn parse_line(line: &str) -> Result<CubePos, ParseError> {
    let split_line: Vec<&str> = line.split(',').collect();

    match split_line[..] {
        [x, y, z] => Ok(CubePos(x.parse()?, y.parse()?, z.parse()?)),
        _ => Err(ParseError::SyntaxError(line.to_string())),
    }
}

fn solve_part_one(contents: &str) -> Result<usize, ParseError> {
    let cubes = get_cubes(contents)?;

    let size_count = get_surface_area(&cubes);

    Ok(size_count)
}

fn get_surface_area(cubes: &[CubePos]) -> usize {
    let mut size_count = 0;
    for (i, cube) in cubes.iter().enumerate() {
        let cubes_already_processed = &cubes[0..i];

        let count_adjacent = cubes_already_processed
            .iter()
            .filter(|other_cube| cube.is_adjacent(other_cube))
            .count();

        size_count = size_count + 6 - 2 * count_adjacent;
    }

    size_count
}

fn solve_part_two(contents: &str, upper_bound: i32) -> Result<usize, ParseError> {
    let cubes = get_cubes(contents)?;

    let surface_area = get_surface_area(&cubes);

    let outside = fill(
        &cubes,
        (-1, upper_bound),
        (-1, upper_bound),
        (-1, upper_bound),
    );

    let mut air_bubbles = vec![];
    for i in -1..upper_bound {
        for j in -1..upper_bound {
            for k in -1..upper_bound {
                if !cubes.contains(&CubePos(i, j, k)) && !outside.contains(&CubePos(i, j, k)) {
                    air_bubbles.push(CubePos(i, j, k));
                }
            }
        }
    }

    Ok(surface_area - get_surface_area(&air_bubbles))
}

fn fill(
    lava: &[CubePos],
    bounds_x: (i32, i32),
    bounds_y: (i32, i32),
    bounds_z: (i32, i32),
) -> Vec<CubePos> {
    let starting_point = CubePos(bounds_x.0, bounds_y.0, bounds_z.0);

    let visited: HashSet<CubePos> = HashSet::new();
    let mutex = Mutex::new(visited);

    fill_recursive(lava, bounds_x, bounds_y, bounds_z, &starting_point, &mutex)
}

fn fill_recursive(
    lava: &[CubePos],
    bounds_x: (i32, i32),
    bounds_y: (i32, i32),
    bounds_z: (i32, i32),
    point: &CubePos,
    visited: &Mutex<HashSet<CubePos>>,
) -> Vec<CubePos> {
    let candidates = point.get_adjacents();
    let mut to_add = vec![];

    for candidate in candidates.iter().filter(|candidate| {
        !visited.lock().unwrap().contains(candidate)
            && !lava.contains(candidate)
            && candidate.0 >= bounds_x.0
            && candidate.0 <= bounds_x.1
            && candidate.1 >= bounds_y.0
            && candidate.1 <= bounds_y.1
            && candidate.2 >= bounds_z.0
            && candidate.2 <= bounds_z.1
    }) {
        to_add.push(*candidate);

        visited.lock().unwrap().insert(*candidate);

        let more = fill_recursive(lava, bounds_x, bounds_y, bounds_z, candidate, visited);
        to_add.extend(more);
    }

    to_add
}

fn main() -> Result<(), ParseError> {
    println!("{}", solve_part_one(INPUT)?);
    println!("{}", solve_part_two(INPUT, 23)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST).unwrap(), 64);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST, 10).unwrap(), 58);
    }
}
