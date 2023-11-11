use std::num::ParseIntError;

use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

const MULTIPLICATION_OF_DIVISORS: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspections: usize,
}

#[derive(Debug)]
struct Game {
    monkeys: Vec<Monkey>,
}

impl Game {
    fn run_round(&mut self, is_relieved: bool) {
        for i in 0..self.monkeys.len() {
            let items = &self.monkeys[i].items.to_vec();

            for item in items {
                let item_with_worry = match self.monkeys[i].operation {
                    Operation::Sum(num) => item + num,
                    Operation::Mul(num) => item * num,
                    Operation::Square => item * item,
                }
                .rem_euclid(MULTIPLICATION_OF_DIVISORS);

                self.monkeys[i].inspections += 1;

                let item_after_bored = if is_relieved {
                    item_with_worry / 3
                } else {
                    item_with_worry
                };

                let Test {
                    divisible_by,
                    true_monkey,
                    false_monkey,
                } = self.monkeys[i].test;

                if item_after_bored % divisible_by as u64 == 0 {
                    self.monkeys[true_monkey].items.push(item_after_bored);
                } else {
                    self.monkeys[false_monkey].items.push(item_after_bored);
                }
            }

            self.monkeys[i].items.clear()
        }
    }
}

#[derive(Debug)]
enum Operation {
    Sum(u64),
    Mul(u64),
    Square,
}

#[derive(Debug, Error)]
#[error("Parse error")]
enum ParseError {
    SyntaxError { line: String },
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug)]
struct Test {
    divisible_by: u32,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse_items(line: &str) -> Result<Vec<u64>, ParseError> {
    let line: Vec<&str> = line
        .trim()
        .split(&[' ', ','])
        .filter(|x| !x.is_empty())
        .collect();
    match &line[..] {
        ["Starting", "items:", items @ ..] => {
            let mut result = Vec::new();

            for item in items {
                result.push(item.parse()?);
            }

            Ok(result)
        }
        _ => Err(ParseError::SyntaxError {
            line: line.join(" "),
        }),
    }
}

fn parse_operation(line: &str) -> Result<Operation, ParseError> {
    let line: Vec<&str> = line.trim().split(' ').collect();
    match &line[..] {
        ["Operation:", "new", "=", "old", "*", "old"] => Ok(Operation::Square),
        ["Operation:", "new", "=", "old", "*", num] => Ok(Operation::Mul(num.parse()?)),
        ["Operation:", "new", "=", "old", "+", num] => Ok(Operation::Sum(num.parse()?)),
        _ => Err(ParseError::SyntaxError {
            line: line.join(" "),
        }),
    }
}

fn parse_test(lines: &[&str]) -> Result<Test, ParseError> {
    let first_line = lines
        .first()
        .ok_or(ParseError::SyntaxError {
            line: "".to_string(),
        })?
        .trim();

    let first_line: Vec<&str> = first_line.split(' ').collect();
    let divisible_by: u32 = match &first_line[..] {
        ["Test:", "divisible", "by", num] => num.parse()?,
        _ => {
            return Err(ParseError::SyntaxError {
                line: first_line.join(" "),
            })
        }
    };

    let true_condition_line = lines
        .get(1)
        .ok_or(ParseError::SyntaxError {
            line: "".to_string(),
        })?
        .trim();
    let true_condition_line: Vec<&str> = true_condition_line.split(' ').collect();

    let true_monkey: usize = match &true_condition_line[..] {
        ["If", "true:", "throw", "to", "monkey", num] => num.parse()?,
        _ => {
            return Err(ParseError::SyntaxError {
                line: true_condition_line.join(" "),
            })
        }
    };

    let false_condition_line = lines
        .get(2)
        .ok_or(ParseError::SyntaxError {
            line: "".to_string(),
        })?
        .trim();
    let false_condition_line: Vec<&str> = false_condition_line.split(' ').collect();

    let false_monkey: usize = match &false_condition_line[..] {
        ["If", "false:", "throw", "to", "monkey", num] => num.parse()?,
        _ => {
            return Err(ParseError::SyntaxError {
                line: false_condition_line.join(" "),
            })
        }
    };

    Ok(Test {
        divisible_by,
        true_monkey,
        false_monkey,
    })
}

fn parse_monkeys(contents: &str) -> Result<Vec<Monkey>, ParseError> {
    let monkeys: Vec<&str> = contents.split("\n\n").collect();

    let mut final_monkeys = Vec::new();
    for (i, monkey) in monkeys.iter().enumerate() {
        let lines: Vec<&str> = monkey.lines().collect();

        let items = parse_items(lines.get(1).ok_or(ParseError::SyntaxError {
            line: "".to_string(),
        })?)?;
        let operation = parse_operation(lines.get(2).ok_or(ParseError::SyntaxError {
            line: "".to_string(),
        })?)?;
        let test = parse_test(&lines[3..])?;

        final_monkeys.push(Monkey {
            id: i,
            items,
            operation,
            test,
            inspections: 0,
        })
    }

    Ok(final_monkeys)
}

fn solve_part_one(contents: &str) -> Result<u64, ParseError> {
    let monkeys = parse_monkeys(contents)?;

    let mut game = Game { monkeys };

    for _ in 0..20 {
        game.run_round(true)
    }

    let mut inspections: Vec<usize> = game
        .monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect();

    inspections.sort();

    Ok(inspections.iter().rev().take(2).product::<usize>() as u64)
}

fn solve_part_two(contents: &str) -> Result<u64, ParseError> {
    let monkeys = parse_monkeys(contents)?;

    let mut game = Game { monkeys };

    for _ in 0..10000 {
        game.run_round(false)
    }

    let mut inspections: Vec<usize> = game
        .monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect();

    inspections.sort();

    Ok(inspections.iter().rev().take(2).product::<usize>() as u64)
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
    fn test_monkey_parsing() {
        let monkeys = parse_monkeys(TEST).unwrap();

        println!("{:?}", monkeys);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST).unwrap(), 10605);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST).unwrap(), 2713310158);
    }
}
