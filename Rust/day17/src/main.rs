use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
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
struct Vector(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(i32, i32);

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
    current_max_height: i32,
    jets: Vec<Jet>,
    current_jet_index: usize,
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
            current_max_height: 0,
            jets: vec![],
            current_jet_index: 0,
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

        self.current_piece_pos = Position(self.current_max_height + height_up, 2);
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
            self.current_max_height = piece_positions
                .iter()
                .map(|p| p.0)
                .max()
                .expect("All pieces are non empty")
                .max(self.current_max_height);
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
                break;
            }
        }

        self.add_piece();
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

fn solve_part_one(contents: &str) -> Result<i32, ParseError> {
    let jets = parse_jets(contents)?;
    let mut wall = Wall::new(jets);

    for _ in 0..2022 {
        wall.drop_until_done();
    }

    Ok(wall.current_max_height + 1)
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
    fn it_works() {
        assert_eq!(solve_part_one(TEST).unwrap(), 3068);
    }
}
