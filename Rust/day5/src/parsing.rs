use crate::crane::{CraneStacks, Movement};
use thiserror::Error;

pub fn parse_file_contents(contents: &str) -> Result<(CraneStacks, Vec<Movement>), ParseError> {
    let lines: Vec<&str> = contents.lines().collect();

    let empty_index = lines.iter().position(|x| x.is_empty()).ok_or(ParseError)?;

    let (first_part, second_part) = lines.split_at(empty_index);
    let second_part = &second_part[1..]; // Removing empty line.

    let crane_stacks = build_crane(first_part)?;

    let movements = second_part
        .iter()
        .copied()
        .map(parse_line_to_movement)
        .collect::<Result<Vec<Movement>, ParseError>>()?;

    Ok((crane_stacks, movements))
}

fn parse_line_to_crane(line: &str) -> Result<Vec<char>, ParseError> {
    line.as_bytes()
        .chunks(4)
        .map(|x| x.get(1).map(|y| *y as char).ok_or(ParseError))
        .collect()
}

fn parse_line_to_movement(line: &str) -> Result<Movement, ParseError> {
    let line: Vec<&str> = line.split(' ').collect();

    match line[..] {
        [_, amount, _, origin, _, destination] => Ok(Movement {
            amount: amount.parse().map_err(|_| ParseError)?,
            origin: origin.parse().map_err(|_| ParseError)?,
            destination: destination.parse().map_err(|_| ParseError)?,
        }),
        _ => Err(ParseError),
    }
}

fn build_crane(lines: &[&str]) -> Result<CraneStacks, ParseError> {
    let last_line = lines.last().ok_or(ParseError)?;
    let n_stacks = last_line.split_whitespace().count();

    let mut crane_stacks = CraneStacks::new_with_stacks(n_stacks);

    for line in lines[..(lines.len() - 1)].iter().rev() {
        let line = parse_line_to_crane(line)?;

        for (i, c) in line.iter().enumerate() {
            if *c != ' ' {
                crane_stacks.stacks[i].push(*c)
            }
        }
    }

    Ok(crane_stacks)
}

#[derive(Debug, Error)]
#[error("Parse error")]
pub struct ParseError;
