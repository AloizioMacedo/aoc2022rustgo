#[derive(Clone, Copy)]
pub enum GameResult {
    Win,
    Draw,
    Loss,
}

impl From<i32> for GameResult {
    fn from(value: i32) -> Self {
        match value.rem_euclid(3) {
            0 => GameResult::Draw,
            1 => GameResult::Win,
            2 => GameResult::Loss,
            _ => unreachable!(),
        }
    }
}

impl From<GameResult> for i32 {
    fn from(result: GameResult) -> Self {
        match result {
            GameResult::Draw => 0,
            GameResult::Win => 1,
            GameResult::Loss => 2,
        }
    }
}

impl TryFrom<char> for GameResult {
    type Error = ParseGameResultError;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'X' => Ok(GameResult::Loss),
            'Y' => Ok(GameResult::Draw),
            'Z' => Ok(GameResult::Win),
            _ => Err(ParseGameResultError),
        }
    }
}

pub struct ParseGameResultError;

impl GameResult {
    pub fn score(&self) -> u32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0,
        }
    }
}
