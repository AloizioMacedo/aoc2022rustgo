use std::collections::HashMap;

use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Piece {
    Horizontal,
    Cross,
    L,
    I,
    Square,
}

impl Piece {
    fn next(&self) -> Self {
        match self {
            Piece::Horizontal => Piece::Cross,
            Piece::Cross => Piece::L,
            Piece::L => Piece::I,
            Piece::I => Piece::Square,
            Piece::Square => Piece::Horizontal,
        }
    }

    fn get_relative_pos(&self) -> PieceRelativePos {
        match self {
            Piece::Horizontal => PieceRelativePos {
                places: vec![Vector(0, 0), Vector(0, 1), Vector(0, 2), Vector(0, 3)],
            },
            Piece::Cross => PieceRelativePos {
                places: vec![
                    Vector(0, 0),
                    Vector(0, 1),
                    Vector(0, 2),
                    Vector(1, 1),
                    Vector(-1, 1),
                ],
            },
            Piece::L => PieceRelativePos {
                places: vec![
                    Vector(0, 0),
                    Vector(0, 1),
                    Vector(0, 2),
                    Vector(1, 2),
                    Vector(2, 2),
                ],
            },
            Piece::I => PieceRelativePos {
                places: vec![Vector(0, 0), Vector(1, 0), Vector(2, 0), Vector(3, 0)],
            },
            Piece::Square => PieceRelativePos {
                places: vec![Vector(0, 0), Vector(0, 1), Vector(1, 0), Vector(1, 1)],
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector(i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(i64, i64);

struct PieceRelativePos {
    places: Vec<Vector>,
}

impl PieceRelativePos {
    fn concretize_at(&self, pos: Position) -> Vec<Position> {
        self.places.iter().map(|v| pos + *v).collect()
    }
}

trait Intersectable {
    fn intersects(&self, other: &Self) -> bool;

    fn out_of_bounds(&self) -> bool;
}

impl Intersectable for Vec<Position> {
    fn intersects(&self, other: &Self) -> bool {
        self.iter().any(|p| other.contains(p))
    }

    fn out_of_bounds(&self) -> bool {
        self.iter().any(|p| p.1 > 6 || p.1 < 0)
    }
}

impl std::ops::Add<Vector> for Position {
    type Output = Position;
    fn add(self, rhs: Vector) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Rock,
    Empty,
}

struct Wall {
    rocks: Vec<Position>,
    current_piece_pos: Position,
    current_piece: Piece,
    current_max_height_idx: usize,
    jets: Vec<Jet>,
    current_jet_index: usize,

    cycle_keys: HashMap<CycleDetector, usize>,
    height_where_cycle_was_detected: Option<usize>,
}

impl Default for Wall {
    fn default() -> Self {
        Wall {
            rocks: vec![
                Position(-1, 0),
                Position(-1, 1),
                Position(-1, 2),
                Position(-1, 3),
                Position(-1, 4),
                Position(-1, 5),
                Position(-1, 6),
            ],
            current_piece: Piece::Horizontal,
            current_piece_pos: Position(3, 2),
            current_max_height_idx: 0,
            jets: vec![],
            current_jet_index: 0,
            cycle_keys: HashMap::new(),
            height_where_cycle_was_detected: None,
        }
    }
}

impl Wall {
    fn new(jets: Vec<Jet>) -> Self {
        Wall {
            jets,
            ..Default::default()
        }
    }

    fn add_piece(&mut self) {
        self.current_piece = self.current_piece.next();

        let height_up = match self.current_piece {
            Piece::Cross => 5,
            _ => 4,
        };

        self.current_piece_pos = Position((self.current_max_height_idx + height_up) as i64, 2);
    }

    fn swoosh(&mut self) {
        let jet = &self.jets[self.current_jet_index];

        match jet {
            Jet::Left => {
                let piece_positions = self
                    .current_piece
                    .get_relative_pos()
                    .concretize_at(self.current_piece_pos);

                let supposed_positions: Vec<Position> = piece_positions
                    .iter()
                    .map(|piece_position| *piece_position + Vector(0, -1))
                    .collect();

                if !supposed_positions.intersects(&self.rocks)
                    && !supposed_positions.out_of_bounds()
                {
                    self.current_piece_pos = self.current_piece_pos + Vector(0, -1);
                }
            }
            Jet::Right => {
                let piece_positions = self
                    .current_piece
                    .get_relative_pos()
                    .concretize_at(self.current_piece_pos);

                let supposed_positions: Vec<Position> = piece_positions
                    .iter()
                    .map(|piece_position| *piece_position + Vector(0, 1))
                    .collect();

                if !supposed_positions.intersects(&self.rocks)
                    && !supposed_positions.out_of_bounds()
                {
                    self.current_piece_pos = self.current_piece_pos + Vector(0, 1);
                }
            }
        }

        self.current_jet_index = (self.current_jet_index + 1) % self.jets.len();
    }

    fn drop_one_step(&mut self) -> Result<(), ()> {
        let piece_positions = self
            .current_piece
            .get_relative_pos()
            .concretize_at(self.current_piece_pos);

        let supposed_positions: Vec<Position> = piece_positions
            .iter()
            .map(|piece_position| *piece_position + Vector(-1, 0))
            .collect();

        if supposed_positions.intersects(&self.rocks) {
            self.current_max_height_idx = piece_positions
                .iter()
                .map(|p| p.0)
                .max()
                .expect("All pieces are non empty")
                .max(self.current_max_height_idx as i64)
                as usize;
            self.rocks.extend(piece_positions);

            Err(())
        } else {
            self.current_piece_pos = self.current_piece_pos + Vector(-1, 0);
            Ok(())
        }
    }

    fn drop_until_done(&mut self) {
        loop {
            // let mut buffer = String::new();
            // std::io::stdin().read_line(&mut buffer).unwrap();

            // println!("Current Piece: {:?}", self.current_piece);
            // println!("Current Piece Position: {:?}", self.current_piece_pos);
            // println!("Current Max Height: {}", self.current_max_height);
            // println!("Current Rocks: {:?}", self.rocks);

            self.swoosh();

            // println!("Current Piece: {:?}", self.current_piece);
            // println!("Current Piece Position: {:?}", self.current_piece_pos);
            // println!("Current Max Height: {}", self.current_max_height);
            // println!("Current Rocks: {:?}", self.rocks);

            let drop_result = self.drop_one_step();
            if drop_result.is_err() {
                let highest_positions = self
                    .rocks
                    .iter()
                    .filter(|rock| rock.0 == self.current_max_height_idx as i64);

                let mut highest_floor = [false; 7];

                for position in highest_positions {
                    highest_floor[position.1 as usize] = true;
                }

                if let Some(height) = self.cycle_keys.get(&CycleDetector {
                    highest_floor,
                    piece: self.current_piece.next(),
                    jet_index: self.current_jet_index,
                }) {
                    self.height_where_cycle_was_detected = Some(*height);
                } else {
                    self.cycle_keys.insert(
                        CycleDetector {
                            highest_floor,
                            piece: self.current_piece.next(),
                            jet_index: self.current_jet_index,
                        },
                        self.current_max_height_idx + 1,
                    );
                }

                break;
            }
        }

        self.add_piece();
    }

    fn run(&mut self, times: usize) -> usize {
        let mut counter = 0;
        let mut cycle_count = None;
        let mut heights = Vec::new();
        let mut delta = None;

        while counter < times {
            let highest_positions: Vec<&Position> = self
                .rocks
                .iter()
                .filter(|rock| rock.0 == *heights.last().unwrap_or(&0) as i64 - 1)
                .collect();

            let mut highest_floor = [false; 7];

            for position in highest_positions.iter() {
                highest_floor[position.1 as usize] = true;
            }

            if let Some(x) = self.height_where_cycle_was_detected {
                let cycle = match cycle_count {
                    Some(count) => count,
                    None => {
                        cycle_count = Some(counter);
                        counter
                    }
                };

                if cycle + counter < times {
                    let idx = heights
                        .iter()
                        .position(|&height| height == x)
                        .expect("Index should exist since a cycle was detected");

                    let n = (times - (idx + 1)) / (counter - (idx + 1));
                    let new_heights = heights[idx..counter].to_vec();

                    let new_delta: Vec<usize> = new_heights
                        .iter()
                        .map(|x| {
                            x - new_heights
                                .first()
                                .expect("New heights should not be empty")
                        })
                        .collect();

                    let new_delta = new_delta[1..].to_vec();
                    let mut new_heights = new_heights[..(new_heights.len() - 1)].to_vec();

                    let last = new_delta.last().expect("Diffs should not be empty");

                    new_heights.iter_mut().for_each(|h| *h += (n - 1) * last);

                    delta = Some(new_delta);
                    heights = new_heights;
                    counter = idx + n * (counter - (idx + 1));
                } else {
                    let delta = delta.clone().expect("Should not be None by here");
                    let last = delta.last().expect("Diffs should not be empty");

                    heights[0..(times - counter)]
                        .iter_mut()
                        .for_each(|h| *h += last);
                    heights = heights[0..(times - counter)].to_vec();

                    counter = times;
                }
            } else {
                self.drop_until_done();
                heights.push(self.current_max_height_idx + 1);

                counter += 1;
            }
        }

        *heights.last().expect("Heights should not be empty")
    }
}

enum Jet {
    Left,
    Right,
}

#[derive(Debug, Error)]
#[error("Parse Error")]
struct ParseError(String);

fn parse_jets(contents: &str) -> Result<Vec<Jet>, ParseError> {
    contents
        .chars()
        .map(|c| match c {
            '<' => Ok(Jet::Left),
            '>' => Ok(Jet::Right),
            _ => Err(ParseError(c.to_string())),
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct CycleDetector {
    highest_floor: [bool; 7],
    piece: Piece,
    jet_index: usize,
}

fn solve_part_one(contents: &str) -> Result<usize, ParseError> {
    let jets = parse_jets(contents)?;
    let mut wall = Wall::new(jets);

    for _ in 0..2022 {
        wall.drop_until_done();
    }

    Ok(wall.current_max_height_idx + 1)
}

fn solve_part_two(contents: &str, n: usize) -> Result<usize, ParseError> {
    let jets = parse_jets(contents)?;
    let mut wall = Wall::new(jets);

    let result = wall.run(n);

    Ok(result)
}

fn main() -> Result<(), ParseError> {
    println!("{}", solve_part_one(INPUT)?);
    println!("{}", solve_part_two(INPUT, 1_000_000_000_000)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = include_str!("../test_input.txt");

    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(TEST).unwrap(), 3068);
    }

    #[test]
    fn part_two() {
        assert_eq!(solve_part_two(TEST, 2022).unwrap(), 3068);
    }

    #[test]
    fn part_two_big() {
        assert_eq!(
            solve_part_two(TEST, 1_000_000_000_000).unwrap(),
            1514285714288
        );
    }
}
