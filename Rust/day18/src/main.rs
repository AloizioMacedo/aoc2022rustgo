use std::num::ParseIntError;

use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

struct CubePos(i32, i32, i32);

impl CubePos {
    fn is_adjacent(&self, other: &CubePos) -> bool {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()) == 1
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

    let mut size_count = 0;
    for (i, cube) in cubes.iter().enumerate() {
        let cubes_already_processed = &cubes[0..i];

        let count_adjacent = cubes_already_processed
            .iter()
            .filter(|other_cube| cube.is_adjacent(other_cube))
            .count();

        size_count += 6 - 2 * count_adjacent;
    }

    Ok(size_count)
}

fn main() -> Result<(), ParseError> {
    println!("{}", solve_part_one(INPUT)?);

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
}
