use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

struct Rucksack<'a> {
    items: &'a str,
}

impl Rucksack<'_> {
    fn get_compartments(&self) -> (&str, &str) {
        let (first, second) = self.items.split_at(self.items.len() / 2);

        (first, second)
    }

    fn get_common_items(&self) -> HashSet<char> {
        let (first, second) = self.get_compartments();

        first.chars().filter(|c| second.contains(*c)).collect()
    }
}

fn get_common_items_across_rucksacks(rs: &[Rucksack]) -> HashSet<char> {
    if let Some(first) = rs.first() {
        first
            .items
            .chars()
            .filter(|c| rs.iter().skip(1).all(|r| r.items.contains(*c)))
            .collect()
    } else {
        HashSet::new()
    }
}

#[derive(Debug)]
struct NoPriority;

trait Priority {
    fn priority(&self) -> Result<u32, NoPriority>;
}

impl Priority for char {
    fn priority(&self) -> Result<u32, NoPriority> {
        match self {
            'a'..='z' => Ok(*self as u32 - 'a' as u32 + 1),
            'A'..='Z' => Ok(*self as u32 - 'A' as u32 + 27),
            _ => Err(NoPriority),
        }
    }
}

fn get_rucksacks(contents: &str) -> Vec<Rucksack> {
    contents
        .lines()
        .map(|line| Rucksack { items: line })
        .collect()
}

fn solve_part_one(contents: &str) -> Result<u32, NoPriority> {
    let rucksacks = get_rucksacks(contents);

    rucksacks
        .iter()
        .map(|rs| {
            rs.get_common_items()
                .iter()
                .map(|c| c.priority())
                .sum::<Result<u32, NoPriority>>()
        })
        .sum()
}

fn solve_part_two(contents: &str) -> Result<u32, NoPriority> {
    let rucksacks = get_rucksacks(contents);

    rucksacks
        .chunks(3)
        .map(|rs| {
            get_common_items_across_rucksacks(rs)
                .iter()
                .map(|c| c.priority())
                .sum::<Result<u32, NoPriority>>()
        })
        .sum()
}

fn main() -> Result<(), NoPriority> {
    println!("{}", solve_part_one(INPUT)?);
    println!("{}", solve_part_two(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT).unwrap(), 157);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT).unwrap(), 70);
    }
}
