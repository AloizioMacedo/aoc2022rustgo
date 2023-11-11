use std::collections::HashSet;

use ndarray::Array2;
use petgraph::{algo::dijkstra, prelude::DiGraphMap};
use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Error)]
#[error("Invalid elevation")]
struct ElevationError {
    wrong_input: char,
}

#[derive(Debug, Error)]
#[error("Graph parsing error.")]
enum GraphParsingError {
    ElevationError(#[from] ElevationError),
    NoStartOrEnd,
}

trait Elevation {
    fn elevation(&self) -> Result<u8, ElevationError>;
}

impl Elevation for char {
    fn elevation(&self) -> Result<u8, ElevationError> {
        match self {
            'a'..='z' => Ok(*self as u8 - b'a'),
            _ => Err(ElevationError { wrong_input: *self }),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Entry {
    Start,
    End,
    Height(u8),
}

impl Entry {
    fn elevation(&self) -> u8 {
        match self {
            Entry::Start => 0,
            Entry::End => 25,
            Entry::Height(x) => *x,
        }
    }
}

impl TryFrom<char> for Entry {
    type Error = ElevationError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Entry::Start),
            'E' => Ok(Entry::End),
            x => Ok(Entry::Height(x.elevation()?)),
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry::Height(0)
    }
}

struct Maze {
    graph: DiGraphMap<(usize, usize), u64>,
    start: (usize, usize),
    end: (usize, usize),
}

fn get_maze(contents: &str) -> Result<(Maze, Array2<Entry>), GraphParsingError> {
    let lines: Vec<&str> = contents.lines().collect();
    let n_rows = lines.len();
    let n_columns = lines[0].len();

    let mut matrix: Array2<Entry> = ndarray::Array2::default((n_rows, n_columns));

    let mut start = None;
    let mut end = None;

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let entry = Entry::try_from(c)?;
            matrix[(i, j)] = entry;

            match entry {
                Entry::Start => start = Some((i, j)),
                Entry::End => end = Some((i, j)),
                _ => (),
            }
        }
    }

    match (start, end) {
        (Some(start), Some(end)) => Ok((
            Maze {
                graph: build_graph(&matrix),
                start,
                end,
            },
            matrix,
        )),
        _ => Err(GraphParsingError::NoStartOrEnd),
    }
}

fn build_graph(matrix: &Array2<Entry>) -> DiGraphMap<(usize, usize), u64> {
    let mut graph = DiGraphMap::new();

    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            let candidates = build_valid_candidates((i, j), matrix.nrows(), matrix.ncols());

            for (h, v) in candidates {
                if matrix[(h, v)].elevation() <= matrix[(i, j)].elevation() + 1 {
                    graph.add_edge((i, j), (h, v), 1);
                }
            }
        }
    }

    graph
}

fn build_valid_candidates(
    origin: (usize, usize),
    n_rows: usize,
    n_cols: usize,
) -> Vec<(usize, usize)> {
    let mut candidates: HashSet<(usize, usize)> = HashSet::from_iter([
        (origin.0 + 1, origin.1),
        (origin.0.saturating_sub(1), origin.1),
        (origin.0, origin.1 + 1),
        (origin.0, origin.1.saturating_sub(1)),
    ]);

    candidates.retain(|(x, y)| *x < n_rows && *y < n_cols);

    candidates.iter().copied().collect()
}

fn solve_part_one(contents: &str) -> Result<u64, GraphParsingError> {
    let (maze, _) = get_maze(contents)?;

    Ok(dijkstra(&maze.graph, maze.start, Some(maze.end), |_| 1)[&maze.end])
}

fn solve_part_two(contents: &str) -> Result<u64, GraphParsingError> {
    let (maze, matrix) = get_maze(contents)?;

    maze.graph
        .nodes()
        .filter(|node| matrix[*node].elevation() == 0)
        .flat_map(|node| {
            dijkstra(&maze.graph, node, Some(maze.end), |_| 1)
                .get(&maze.end)
                .copied()
        })
        .min()
        .ok_or(GraphParsingError::NoStartOrEnd)
}

fn main() -> Result<(), GraphParsingError> {
    println!("{}", solve_part_one(INPUT)?);
    println!("{}", solve_part_two(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = include_str!("../test_input.txt");

    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(TEST).unwrap(), 31)
    }

    #[test]
    fn part_two() {
        assert_eq!(solve_part_two(TEST).unwrap(), 29)
    }
}
