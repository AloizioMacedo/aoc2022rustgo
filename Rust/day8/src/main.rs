use ndarray::Array2;

const INPUT: &str = include_str!("../input.txt");

struct Forest {
    trees: Array2<u32>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Forest {
    fn is_visible_from_direction(&self, i: usize, j: usize, direction: Direction) -> bool {
        match direction {
            Direction::Up => (0..i).all(|k| self.trees[[k, j]] < self.trees[[i, j]]),
            Direction::Down => {
                ((i + 1)..self.trees.nrows()).all(|k| self.trees[[k, j]] < self.trees[[i, j]])
            }
            Direction::Left => (0..j).all(|k| self.trees[[i, k]] < self.trees[[i, j]]),
            Direction::Right => {
                ((j + 1)..self.trees.ncols()).all(|k| self.trees[[i, k]] < self.trees[[i, j]])
            }
        }
    }

    fn get_view_from_direction(&self, i: usize, j: usize, direction: Direction) -> usize {
        match direction {
            Direction::Up => {
                if let Some(pos) = (0..i)
                    .rev()
                    .position(|k| self.trees[[k, j]] >= self.trees[[i, j]])
                {
                    pos + 1
                } else {
                    i
                }
            }
            Direction::Down => {
                if let Some(pos) = ((i + 1)..self.trees.nrows())
                    .position(|k| self.trees[[k, j]] >= self.trees[[i, j]])
                {
                    pos + 1
                } else {
                    self.trees.nrows() - (i + 1)
                }
            }
            Direction::Left => {
                if let Some(pos) = (0..j)
                    .rev()
                    .position(|k| self.trees[[i, k]] >= self.trees[[i, j]])
                {
                    pos + 1
                } else {
                    j
                }
            }
            Direction::Right => {
                if let Some(pos) = ((j + 1)..self.trees.ncols())
                    .position(|k| self.trees[[i, k]] >= self.trees[[i, j]])
                {
                    pos + 1
                } else {
                    self.trees.ncols() - (j + 1)
                }
            }
        }
    }

    fn get_view_score(&self, i: usize, j: usize) -> usize {
        self.get_view_from_direction(i, j, Direction::Up)
            * self.get_view_from_direction(i, j, Direction::Down)
            * self.get_view_from_direction(i, j, Direction::Left)
            * self.get_view_from_direction(i, j, Direction::Right)
    }

    fn is_visible_from_outside(&self, i: usize, j: usize) -> bool {
        self.is_visible_from_direction(i, j, Direction::Up)
            || self.is_visible_from_direction(i, j, Direction::Down)
            || self.is_visible_from_direction(i, j, Direction::Left)
            || self.is_visible_from_direction(i, j, Direction::Right)
    }
}

#[derive(Debug)]
struct ParseError;

fn parse_forest(contents: &str) -> Result<Forest, ParseError> {
    let lines: Vec<&str> = contents.lines().collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut trees: Array2<u32> = Array2::zeros((rows, cols));

    for (i, line) in lines.iter().enumerate() {
        let line = line.as_bytes();
        for j in 0..cols {
            trees[[i, j]] = (line[j] as char).to_digit(10).ok_or(ParseError)?;
        }
    }

    Ok(Forest { trees })
}

fn solve_part_one(contents: &str) -> Result<usize, ParseError> {
    let forest = parse_forest(contents)?;

    Ok(forest
        .trees
        .indexed_iter()
        .filter(|((i, j), _)| forest.is_visible_from_outside(*i, *j))
        .count())
}

fn solve_part_two(contents: &str) -> Result<usize, ParseError> {
    let forest = parse_forest(contents)?;

    Ok(forest
        .trees
        .indexed_iter()
        .map(|((i, j), _)| forest.get_view_score(i, j))
        .max()
        .unwrap_or(0))
}

fn main() -> Result<(), ParseError> {
    println!("{}", solve_part_one(INPUT)?);
    println!("{:?}", solve_part_two(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = include_str!("../test_input.txt");

    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(TEST).unwrap(), 21);
    }

    #[test]
    fn part_two() {
        assert_eq!(solve_part_two(TEST).unwrap(), 8);
    }
}
