use std::{
    collections::HashSet,
    ops::{AddAssign, Sub},
};

const INPUT: &str = include_str!("../input.txt");

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position(i32, i32);

#[derive(Debug, Clone, Copy)]
struct Vector(i32, i32);

impl AddAssign<Vector> for Position {
    fn add_assign(&mut self, rhs: Vector) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Position {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

struct BigRope {
    knots: Vec<Position>,

    visited: HashSet<Position>,
}

impl BigRope {
    fn new(knots: usize) -> Self {
        Self {
            knots: vec![Position(0, 0); knots],
            visited: HashSet::from([Position(0, 0)]),
        }
    }

    fn mov(&mut self, dir: &Direction) {
        if self.knots.is_empty() {
            return;
        }

        match dir {
            Direction::Up => {
                self.knots[0].1 += -1;
            }
            Direction::Down => {
                self.knots[0].1 += 1;
            }
            Direction::Left => {
                self.knots[0].0 += -1;
            }
            Direction::Right => {
                self.knots[0].0 += 1;
            }
        };

        for i in 0..(self.knots.len() - 1) {
            let delta_head_tail = self.knots[i] - self.knots[i + 1];

            if delta_head_tail.0.abs().max(delta_head_tail.1.abs()) >= 2 {
                self.knots[i + 1] += Vector(delta_head_tail.0.signum(), delta_head_tail.1.signum());
            }
        }

        if let Some(last) = self.knots.last() {
            self.visited.insert(*last);
        }
    }
}

struct Rope {
    head: Position,
    tail: Position,

    visited: HashSet<Position>,
}

#[derive(Debug)]
struct ParseError;

impl Default for Rope {
    fn default() -> Self {
        Self {
            head: Position(0, 0),
            tail: Position(0, 0),
            visited: HashSet::from([Position(0, 0)]),
        }
    }
}

impl Rope {
    fn mov(&mut self, dir: &Direction) {
        let delta_head_tail = match dir {
            Direction::Up => {
                self.head.1 += -1;
                self.head - self.tail
            }
            Direction::Down => {
                self.head.1 += 1;
                self.head - self.tail
            }
            Direction::Left => {
                self.head.0 += -1;
                self.head - self.tail
            }
            Direction::Right => {
                self.head.0 += 1;
                self.head - self.tail
            }
        };

        if delta_head_tail.0.abs().max(delta_head_tail.1.abs()) >= 2 {
            self.tail += Vector(delta_head_tail.0.signum(), delta_head_tail.1.signum());

            self.visited.insert(self.tail);
        }
    }
}

struct Movement {
    dir: Direction,
    steps: usize,
}

fn parse_movements(contents: &str) -> Result<Vec<Movement>, ParseError> {
    contents.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Movement, ParseError> {
    let line: Vec<&str> = line.split(' ').collect();
    match line[..] {
        [dir, steps] => Ok(Movement {
            dir: Direction::try_from(dir.as_bytes()[0] as char)?,
            steps: steps.parse().map_err(|_| ParseError)?,
        }),
        _ => Err(ParseError),
    }
}

fn solve_part_one(contents: &str) -> Result<usize, ParseError> {
    let movements = parse_movements(contents)?;

    let mut snake = Rope::default();

    movements.iter().for_each(|mov| {
        for _ in 0..mov.steps {
            snake.mov(&mov.dir);
        }
    });

    Ok(snake.visited.len())
}

fn solve_part_two(contents: &str) -> Result<usize, ParseError> {
    let movements = parse_movements(contents)?;

    let mut snake = BigRope::new(10);

    movements.iter().for_each(|mov| {
        for _ in 0..mov.steps {
            snake.mov(&mov.dir);
        }
    });

    Ok(snake.visited.len())
}

fn main() -> Result<(), ParseError> {
    println!("{}", solve_part_one(INPUT)?);
    println!("{}", solve_part_two(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_one() {
        let contents = TEST;

        assert_eq!(solve_part_one(contents).unwrap(), 13);
    }

    #[test]
    fn test_part_two() {
        let contents = TEST;

        assert_eq!(solve_part_two(contents).unwrap(), 1);
    }
}
