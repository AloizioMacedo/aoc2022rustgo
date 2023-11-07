use day2::{
    choice::{Choice, ParseChoiceError},
    game::GameResult,
};

const CONTENT: &str = include_str!("../input.txt");

fn _parse_choices(contents: &str) -> Result<Vec<(Choice, Choice)>, ParseChoiceError> {
    contents
        .lines()
        .map(|line| match line.as_bytes() {
            [a, _, b] => {
                let first = Choice::try_from(*a as char);
                let second = Choice::try_from(*b as char);

                match (first, second) {
                    (Ok(first), Ok(second)) => Ok((first, second)),
                    _ => Err(ParseChoiceError),
                }
            }
            _ => Err(ParseChoiceError),
        })
        .collect()
}

// In a real program, we could use the thiserror crate to create better error types.
#[derive(Debug)]
struct ParseError;

fn _parse_choices_and_necessary_results(
    contents: &str,
) -> Result<Vec<(Choice, GameResult)>, ParseError> {
    contents
        .lines()
        .map(|line| match line.as_bytes() {
            [a, _, b] => {
                let first = Choice::try_from(*a as char);
                let second = GameResult::try_from(*b as char);

                match (first, second) {
                    (Ok(first), Ok(second)) => Ok((first, second)),
                    _ => Err(ParseError),
                }
            }
            _ => Err(ParseError),
        })
        .collect()
}

// Unites the above parsing functionalities using generics.
fn parse_file_contents<T, U>(contents: &str) -> Result<Vec<(T, U)>, ParseError>
where
    T: TryFrom<char>,
    U: TryFrom<char>,
{
    contents
        .lines()
        .map(|line| match line.as_bytes() {
            [a, _, b] => {
                let first = T::try_from(*a as char);
                let second = U::try_from(*b as char);

                match (first, second) {
                    (Ok(first), Ok(second)) => Ok((first, second)),
                    _ => Err(ParseError),
                }
            }
            _ => Err(ParseError),
        })
        .collect()
}

fn solve_part_one(contents: &str) -> Result<u32, ParseError> {
    let choices = parse_file_contents::<Choice, Choice>(contents)?;

    Ok(choices.iter().fold(0, |acc, (their_choice, my_choice)| {
        acc + my_choice.pit_against(their_choice).score() + my_choice.score()
    }))
}

fn solve_part_two(contents: &str) -> Result<u32, ParseError> {
    let choices = parse_file_contents::<Choice, GameResult>(contents)?;

    Ok(choices
        .iter()
        .fold(0, |acc, (their_choice, intended_result)| {
            let my_choice = their_choice.get_choice_to_ensure_result_against(intended_result);

            acc + my_choice.score() + intended_result.score()
        }))
}

fn main() -> Result<(), ParseError> {
    println!("{}", solve_part_one(CONTENT)?);
    println!("{}", solve_part_two(CONTENT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONTENT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_parse_choices() {
        let choices = _parse_choices(TEST_CONTENT).unwrap();

        assert_eq!(
            choices,
            vec![
                (Choice::Rock, Choice::Paper),
                (Choice::Paper, Choice::Rock),
                (Choice::Scissors, Choice::Scissors)
            ]
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST_CONTENT).unwrap(), 15);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST_CONTENT).unwrap(), 12);
    }
}
