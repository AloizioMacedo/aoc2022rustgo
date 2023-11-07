use crate::game::GameResult;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Choice {
    type Error = ParseChoiceError;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'A' | 'X' => Ok(Choice::Rock),
            'B' | 'Y' => Ok(Choice::Paper),
            'C' | 'Z' => Ok(Choice::Scissors),
            _ => Err(ParseChoiceError),
        }
    }
}

#[derive(Debug)]
pub struct ParseChoiceError;

impl From<Choice> for i32 {
    fn from(choice: Choice) -> Self {
        match choice {
            Choice::Rock => 0,
            Choice::Paper => 1,
            Choice::Scissors => 2,
        }
    }
}

impl From<i32> for Choice {
    fn from(value: i32) -> Self {
        match value.rem_euclid(3) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            2 => Choice::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Choice {
    pub fn score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    pub fn get_choice_to_ensure_result_against(&self, result: &GameResult) -> Choice {
        (i32::from(*self) + i32::from(*result)).into()
    }

    pub fn pit_against(&self, other: &Choice) -> GameResult {
        (i32::from(*self) - i32::from(*other)).into()
    }
}
