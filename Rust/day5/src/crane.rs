use thiserror::Error;

#[derive(Debug)]
pub struct CraneStacks {
    pub stacks: Vec<Vec<char>>,
}

impl CraneStacks {
    pub fn new_with_stacks(n_stacks: usize) -> Self {
        let mut stacks = vec![];

        for _ in 0..n_stacks {
            stacks.push(Vec::new());
        }

        Self { stacks }
    }

    pub fn mov(&mut self, movement: Movement) -> Result<(), MovementError> {
        for _ in 0..movement.amount {
            let item = self.stacks[movement.origin - 1]
                .pop()
                .ok_or(MovementError)?;

            self.stacks[movement.destination - 1].push(item);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
#[error("Debug error")]
pub struct MovementError;

pub struct Movement {
    pub amount: usize,
    pub origin: usize,
    pub destination: usize,
}
