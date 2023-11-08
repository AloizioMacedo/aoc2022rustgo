use std::num::ParseIntError;

fn get_calories(file_contents: &str) -> Result<Vec<u32>, ParseIntError> {
    if file_contents.is_empty() {
        return Ok(vec![]);
    }

    let mut calories = vec![0];

    for line in file_contents.lines() {
        if line.is_empty() {
            calories.push(0);
        } else {
            let last = calories
                .last_mut()
                .expect("Calories vec should never be empty as it is initialized with 0");

            *last += line.parse::<u32>()?;
        }
    }

    Ok(calories)
}

pub fn solve_part_one(contents: &str) -> Result<Option<u32>, ParseIntError> {
    let calories = get_calories(contents)?;

    Ok(calories.iter().max().copied())
}

pub fn solve_part_two(contents: &str) -> Result<u32, ParseIntError> {
    let mut calories = get_calories(contents)?;

    calories.sort();

    Ok(calories.iter().rev().take(3).sum())
}

fn main() -> Result<(), ParseIntError> {
    // Embeds the content of input.txt inside the final binary, using it as a reference.
    let contents = include_str!("../input.txt");

    let solution1 = solve_part_one(contents)?;
    println!("{:?}", solution1);

    let solution2 = solve_part_two(contents)?;
    println!("{:?}", solution2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_file() {
        // Allocates the content of test_input.txt dynamically in the heap, differently
        // from how the content of input.txt ends up in the final binary by using the
        // include_str! macro.
        let file_contents =
            std::fs::read_to_string("test_input.txt").expect("Test file should exist");

        let expected = vec![6000, 4000, 11000, 24000, 10000];

        assert_eq!(get_calories(&file_contents).unwrap(), expected);
        assert_eq!(get_calories("").unwrap(), vec![]);
    }

    #[test]
    fn test_part_one() {
        let file_contents =
            std::fs::read_to_string("test_input.txt").expect("Test file should exist");

        let expected = 24000;
        assert_eq!(solve_part_one(&file_contents).unwrap().unwrap(), expected);
    }

    #[test]
    fn test_part_two() {
        let file_contents =
            std::fs::read_to_string("test_input.txt").expect("Test file should exist");

        let expected = 45000;
        assert_eq!(solve_part_two(&file_contents).unwrap(), expected);
    }
}
