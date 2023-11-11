use std::num::ParseIntError;

use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

struct CPU {
    current_cycle: i32,
    register: i32,
    values_history: Vec<i32>,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            current_cycle: 1,
            register: 1,
            values_history: vec![i32::MAX],
        }
    }
}

impl CPU {
    fn process_command(&mut self, command: &Command) {
        match command {
            Command::Noop => {
                // Start of the cycle, value is the same.
                self.values_history.push(self.register);

                // Ends the cycle.
                self.current_cycle += 1;
            }
            Command::Addx(x) => {
                // Start of the cycle, value is the same.
                self.values_history.push(self.register);

                // Ends the cycle.
                self.current_cycle += 1;

                // Start of the cycle, value is the same.
                self.values_history.push(self.register);

                // Ends the cycle.
                self.current_cycle += 1;

                // Value change after the end of second cycle.
                self.register += x;
            }
        }
    }

    fn get_history(&self) -> Vec<i32> {
        [self.values_history.to_vec(), vec![self.register]].concat()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Noop,
    Addx(i32),
}

fn parse_line(line: &str) -> Result<Command, ParseError> {
    let line: Vec<&str> = line.split(' ').collect();

    match line[..] {
        ["noop"] => Ok(Command::Noop),
        ["addx", x] => Ok(Command::Addx(x.parse()?)),
        _ => Err(ParseError::SyntaxError),
    }
}

#[derive(Error, Debug)]
#[error("Parse Error")]
enum ParseError {
    SyntaxError,
    ParseIntError(#[from] ParseIntError),
}

fn get_commands(contents: &str) -> Result<Vec<Command>, ParseError> {
    contents.lines().map(parse_line).collect()
}

fn solve_part_one(contents: &str) -> Result<i32, ParseError> {
    let commands = get_commands(contents)?;

    let mut cpu = CPU::default();

    for command in commands {
        cpu.process_command(&command);
    }

    Ok(cpu
        .get_history()
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .fold(0, |acc, (i, v)| acc + (i as i32) * v))
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
    fn test_get_commands() {
        assert_eq!(
            get_commands(TEST).unwrap(),
            vec![Command::Noop, Command::Addx(3), Command::Addx(-5),]
        )
    }

    #[test]
    fn test_part_one() {
        let commands = get_commands(TEST).unwrap();

        let mut cpu = CPU::default();

        for command in commands {
            cpu.process_command(&command);
        }

        assert_eq!(cpu.register, -1)
    }
}
