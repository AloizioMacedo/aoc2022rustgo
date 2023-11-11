use std::{fmt::Display, num::ParseIntError};

use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

enum Pixel {
    Lit,
    Dark,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pixel::Lit => write!(f, "#"),
            Pixel::Dark => write!(f, "."),
        }
    }
}

struct Cpu {
    current_cycle: usize,
    register: i32,
    values_history: Vec<i32>,
    pixels: Vec<Pixel>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            current_cycle: 1,
            register: 1,
            values_history: vec![i32::MAX],
            pixels: vec![],
        }
    }
}

impl Cpu {
    fn draw_crt(&self) {
        for row in self.pixels.chunks(40) {
            for pixel in row {
                print!("{}", pixel);
            }
            println!();
        }
    }

    fn draw_pixel(&mut self) {
        if ((self.current_cycle - 1).rem_euclid(40) + 1).abs_diff((self.register + 1) as usize) <= 1
        {
            self.pixels.push(Pixel::Lit)
        } else {
            self.pixels.push(Pixel::Dark)
        }
    }

    fn process_command(&mut self, command: &Command) {
        match command {
            Command::Noop => {
                // Start of the cycle, value is the same.
                self.values_history.push(self.register);

                self.draw_pixel();

                // Ends the cycle.
                self.current_cycle += 1;
            }
            Command::Addx(x) => {
                // Start of the cycle, value is the same.
                self.values_history.push(self.register);

                self.draw_pixel();

                // Ends the cycle.
                self.current_cycle += 1;

                // Start of the cycle, value is the same.
                self.values_history.push(self.register);

                self.draw_pixel();

                // Ends the cycle.
                self.current_cycle += 1;

                // Value change after the end of second cycle.
                self.register += x;
            }
        }
    }

    fn get_history(&self) -> impl Iterator<Item = i32> + '_ {
        let history = self.values_history.iter().copied();
        let register = self.register;

        history.chain(std::iter::once(register))
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

    let mut cpu = Cpu::default();

    for command in commands {
        cpu.process_command(&command);
    }

    Ok(cpu
        .get_history()
        .enumerate()
        .skip(20)
        .step_by(40)
        .fold(0, |acc, (i, v)| acc + (i as i32) * v))
}

fn solve_part_two(contents: &str) -> Result<(), ParseError> {
    let commands = get_commands(contents)?;

    let mut cpu = Cpu::default();

    for command in commands {
        cpu.process_command(&command);
    }

    cpu.draw_crt();

    Ok(())
}

fn main() -> Result<(), ParseError> {
    println!("{}", solve_part_one(INPUT)?);
    solve_part_two(INPUT)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = include_str!("../test_input.txt");
    const TEST2: &str = include_str!("../test_input2.txt");

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

        let mut cpu = Cpu::default();

        for command in commands {
            cpu.process_command(&command);
        }

        assert_eq!(cpu.register, -1)
    }

    #[test]
    fn test_part_two() {
        solve_part_two(TEST2).unwrap();
    }
}
