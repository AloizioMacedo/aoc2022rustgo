use day5::{
    crane::MovementError,
    parsing::{parse_file_contents, ParseError},
};
use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

fn solve_part_one(contents: &str) -> Result<String, ExecutionError> {
    let (mut crane_stacks, movements) = parse_file_contents(contents)?;

    for movement in movements {
        crane_stacks.mov(movement)?;
    }

    Ok(crane_stacks
        .stacks
        .iter()
        .flat_map(|stack| stack.last())
        .collect())
}

#[derive(Error, Debug)]
enum ExecutionError {
    #[error("Parse error")]
    ParseError(#[from] ParseError),

    #[error("Movement error")]
    MovementError(#[from] MovementError),
}

fn main() -> Result<(), ExecutionError> {
    println!("{}", solve_part_one(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("../test_input.txt");

    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(TEST).unwrap(), "CMZ");
    }
}
