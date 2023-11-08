use std::num::ParseIntError;

use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn contains(&self, other: &Interval) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Interval) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
            || (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

fn get_intervals(file_contents: &str) -> Result<Vec<(Interval, Interval)>, ParseError> {
    file_contents.lines().map(parse_line).collect()
}

fn solve_part_one(file_contents: &str) -> Result<usize, ParseError> {
    let intervals = get_intervals(file_contents)?;

    Ok(intervals
        .iter()
        .filter(|(i1, i2)| i1.contains(i2) || i2.contains(i1))
        .count())
}

fn solve_part_two(file_contents: &str) -> Result<usize, ParseError> {
    let intervals = get_intervals(file_contents)?;

    Ok(intervals.iter().filter(|(i1, i2)| i1.overlaps(i2)).count())
}

fn parse_line(line: &str) -> Result<(Interval, Interval), ParseError> {
    let split_line: Vec<&str> = line.split(&[',', '-']).collect();

    match split_line[..] {
        [a1, a2, b1, b2] => Ok((
            Interval {
                start: a1.parse()?,
                end: a2.parse()?,
            },
            Interval {
                start: b1.parse()?,
                end: b2.parse()?,
            },
        )),
        _ => Err(ParseError::IncorrectFormat),
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Error when parsing int")]
    ParseIntError(#[from] ParseIntError),

    #[error("Incorrect format")]
    IncorrectFormat,
}

fn main() -> Result<(), ParseError> {
    println!("{}", solve_part_one(INPUT)?);
    println!("{}", solve_part_two(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = include_str!("../test_input1.txt");
    const TEST2: &str = include_str!("../test_input2.txt");

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("2-4,6-8").unwrap(),
            (Interval { start: 2, end: 4 }, Interval { start: 6, end: 8 })
        );

        assert!(parse_line("2-4,6-12a").is_err());
        assert!(parse_line("").is_err());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST1).unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST2).unwrap(), 4);
    }
}
