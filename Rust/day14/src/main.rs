use std::fmt::Display;

use ndarray::{Array2, Axis};
use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

const SAND_ORIGIN: (usize, usize) = (0, 500);
const THRESHOLD_ROW: usize = 300;
const BOUNDS: usize = 1000;

struct Cave {
    wall: Array2<Entry>,
    current_sand_position: (usize, usize),
    sands: usize,
}

impl Default for Cave {
    fn default() -> Self {
        let mut wall = Array2::default((BOUNDS, BOUNDS));
        wall[SAND_ORIGIN] = Entry::Sand;

        Cave {
            wall,
            current_sand_position: SAND_ORIGIN,
            sands: 1,
        }
    }
}

#[derive(Error, Debug)]
#[error("No more sand")]
struct NoMoreSand;

impl Cave {
    fn print(&self) {
        let mut min_i = usize::MAX;
        let mut min_j = usize::MAX;

        let mut max_i = usize::MIN;
        let mut max_j = usize::MIN;
        for ((i, j), entry) in self.wall.indexed_iter() {
            if matches!(entry, Entry::Rock | Entry::Sand) {
                min_i = i.min(min_i);
                min_j = j.min(min_j);

                max_i = i.max(max_i);
                max_j = j.max(max_j);
            }
        }

        let s = self
            .wall
            .select(Axis(0), &Vec::from_iter(min_i..(max_i + 1)));
        let s = s.select(Axis(1), &Vec::from_iter(min_j..(max_j + 1)));

        println!("{}", s);
    }

    fn drop_sand(&mut self) -> Result<(), NoMoreSand> {
        let sand = self.current_sand_position;

        if matches!(self.wall[(sand.0 + 1, sand.1)], Entry::Empty) {
            self.wall[self.current_sand_position] = Entry::Empty;
            self.current_sand_position = (sand.0 + 1, sand.1);
            self.wall[self.current_sand_position] = Entry::Sand;
        } else if matches!(self.wall[(sand.0 + 1, sand.1 - 1)], Entry::Empty) {
            self.wall[self.current_sand_position] = Entry::Empty;
            self.current_sand_position = (sand.0 + 1, sand.1 - 1);
            self.wall[self.current_sand_position] = Entry::Sand;
        } else if matches!(self.wall[(sand.0 + 1, sand.1 + 1)], Entry::Empty) {
            self.wall[self.current_sand_position] = Entry::Empty;
            self.current_sand_position = (sand.0 + 1, sand.1 + 1);
            self.wall[self.current_sand_position] = Entry::Sand;
        } else {
            if sand == SAND_ORIGIN {
                return Err(NoMoreSand);
            }

            self.wall[SAND_ORIGIN] = Entry::Sand;
            self.current_sand_position = SAND_ORIGIN;
            self.sands += 1;
        }

        if self.current_sand_position.0 > THRESHOLD_ROW {
            return Err(NoMoreSand);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum Entry {
    Rock,
    Sand,

    #[default]
    Empty,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Rock => write!(f, "#"),
            Entry::Sand => write!(f, "o"),
            Entry::Empty => write!(f, "."),
        }
    }
}

#[derive(Error, Debug)]
#[error("Parsing error")]
struct ParsingError(String);

fn build_cave(contents: &str) -> Result<Cave, ParsingError> {
    let mut cave = Cave::default();

    contents
        .lines()
        .try_for_each(|line| parse_line(&mut cave, line))?;

    Ok(cave)
}

fn build_cave_with_floor(contents: &str) -> Result<Cave, ParsingError> {
    let mut cave = Cave::default();

    contents
        .lines()
        .try_for_each(|line| parse_line(&mut cave, line))?;

    let mut max_i = usize::MIN;
    for ((i, _), entry) in cave.wall.indexed_iter() {
        if matches!(entry, Entry::Rock | Entry::Sand) {
            max_i = i.max(max_i);
        }
    }

    for j in 0..BOUNDS {
        cave.wall[(max_i + 2, j)] = Entry::Rock;
    }

    Ok(cave)
}

fn parse_line(cave: &mut Cave, line: &str) -> Result<(), ParsingError> {
    let line: Vec<&str> = line.split(" -> ").collect();
    let line: Vec<Vec<&str>> = line
        .iter()
        .map(|pair| pair.split(',').collect::<Vec<&str>>())
        .collect();

    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for pair in line {
        match pair[..] {
            [x, y] => pairs.push((
                y.parse().map_err(|_| ParsingError(y.to_string() + y))?,
                x.parse().map_err(|_| ParsingError(x.to_string() + x))?,
            )),
            _ => return Err(ParsingError(pair.join(","))),
        }
    }

    let wall = &mut cave.wall;

    for (origin, destination) in pairs.iter().zip(pairs.iter().skip(1)) {
        if origin.0 == destination.0 {
            let (first, second) = if origin.1 <= destination.1 {
                (origin.1, destination.1)
            } else {
                (destination.1, origin.1)
            };

            for j in first..=second {
                wall[[origin.0, j]] = Entry::Rock;
            }
        } else if origin.1 == destination.1 {
            let (first, second) = if origin.0 <= destination.0 {
                (origin.0, destination.0)
            } else {
                (destination.0, origin.0)
            };

            for i in first..=second {
                wall[[i, origin.1]] = Entry::Rock;
            }
        } else {
            panic!("Diagonal lines not supported");
        }
    }

    Ok(())
}

fn solve_part_one(contents: &str) -> Result<usize, ParsingError> {
    let mut cave = build_cave(contents)?;

    loop {
        let drop_result = cave.drop_sand();

        if drop_result.is_err() {
            break;
        }
    }

    Ok(cave.sands - 1)
}

fn solve_part_two(contents: &str) -> Result<usize, ParsingError> {
    let mut cave = build_cave_with_floor(contents)?;

    loop {
        let drop_result = cave.drop_sand();

        if drop_result.is_err() {
            break;
        }
    }

    Ok(cave.sands)
}

fn main() -> Result<(), ParsingError> {
    println!("{}", solve_part_one(INPUT)?);
    println!("{}", solve_part_two(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    const TEST: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST).unwrap(), 24);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST).unwrap(), 93);
    }

    #[ignore]
    #[test]
    fn visualize_sand_drop() {
        let mut cave = build_cave(TEST).unwrap();

        for _ in 0..300 {
            let drop = cave.drop_sand();

            if drop.is_err() {
                break;
            }

            sleep(Duration::from_millis(5));
            print!("{}[2J", 27 as char);
            cave.print();
        }
    }
}
