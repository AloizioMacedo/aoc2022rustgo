use std::num::ParseIntError;

/// Straightforward implementation that hides some possible errors inside the function.
fn _get_calories(file_contents: &str) -> Vec<u32> {
    let mut calories = vec![0];

    for line in file_contents.lines() {
        if line.is_empty() {
            calories.push(0);
        } else {
            let last_idx = calories.len() - 1;
            calories[last_idx] += line.parse::<u32>().unwrap();
        }
    }

    calories
}

/// Straightforward implementation that has useful messages for the errors.
///
/// However, still fundamentally hides the errors inside the function.
fn _get_calories_with_expects(file_contents: &str) -> Vec<u32> {
    let mut calories = vec![0];

    for line in file_contents.lines() {
        if line.is_empty() {
            calories.push(0);
        } else {
            *calories.last_mut().expect("Calories should not be empty") += line
                .parse::<u32>()
                .expect("Non-empty line should be parseable to a number");
        }
    }

    calories
}

/// Is more explicit regarding the fact that the function can have an error.
///
/// Does not correctly handle the empty file scenario.
fn _get_calories_with_result_output(file_contents: &str) -> Result<Vec<u32>, ParseIntError> {
    let mut calories = vec![0];

    for line in file_contents.lines() {
        if line.is_empty() {
            calories.push(0);
        } else {
            *calories.last_mut().expect("Calories should not be empty") += line.parse::<u32>()?;
        }
    }

    Ok(calories)
}

/// This version correctly handles the case where the file can be empty
///
/// All other functions above will have a lingering 0 which is not a good representation
/// of the empty case.
///
/// The fact that this function handles the empty file correctly is unrelated to the
/// fact that it is using try_fold. Try_fold was used just to illustrate a different,
/// iterator-based approach.
fn get_calories_with_fold_and_result_output(
    file_contents: &str,
) -> Result<Vec<u32>, ParseIntError> {
    file_contents.lines().try_fold(vec![], |mut acc, line| {
        if line.is_empty() {
            acc.push(0);

            Ok(acc)
        } else {
            match line.parse::<u32>() {
                Ok(n) => {
                    if let Some(last) = acc.last_mut() {
                        *last += n;
                    } else {
                        acc.push(n);
                    }

                    Ok(acc)
                }
                Err(e) => Err(e),
            }
        }
    })
}

/// Hides the errors for the solution inside the function.
fn _solve_part_one_with_expects(contents: &str) -> u32 {
    let calories =
        get_calories_with_fold_and_result_output(contents).expect("Should be able to parse ints");

    *calories.iter().max().expect("Calories should not be empty")
}

/// Solves the first part of the problem.
pub fn solve_part_one_with_result(contents: &str) -> Result<Option<u32>, ParseIntError> {
    let calories = get_calories_with_fold_and_result_output(contents)?;

    Ok(calories.iter().max().copied())
}

/// Solves the second part of the problem.
pub fn solve_part_two_with_result(contents: &str) -> Result<u32, ParseIntError> {
    let mut calories = get_calories_with_fold_and_result_output(contents)?;

    calories.sort();

    Ok(calories.iter().rev().take(3).sum())
}

fn main() -> Result<(), ParseIntError> {
    // Embeds the content of input.txt inside the final binary, using it as a reference.
    let contents = include_str!("../input.txt");

    let solution1 = solve_part_one_with_result(contents)?;
    println!("{:?}", solution1);

    let solution2 = solve_part_two_with_result(contents)?;
    println!("{:?}", solution2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_file() {
        // Allocates the content of test_input.txt dynamically in the heap, differently
        // from how the content of input.txt ends up in the final binary.
        let file_contents =
            std::fs::read_to_string("test_input.txt").expect("Test file should exist");

        let expected = vec![6000, 4000, 11000, 24000, 10000];

        assert_eq!(_get_calories(&file_contents), expected);
        assert_eq!(_get_calories_with_expects(&file_contents), expected);
        assert_eq!(
            _get_calories_with_result_output(&file_contents).unwrap(),
            expected
        );
        assert_eq!(
            get_calories_with_fold_and_result_output(&file_contents).unwrap(),
            expected
        );
    }

    #[test]
    fn test_part_one() {
        let file_contents =
            std::fs::read_to_string("test_input.txt").expect("Test file should exist");

        let expected = 24000;
        assert_eq!(_solve_part_one_with_expects(&file_contents), expected);
        assert_eq!(
            solve_part_one_with_result(&file_contents).unwrap().unwrap(),
            expected
        );
    }

    #[test]
    fn test_part_two() {
        let file_contents =
            std::fs::read_to_string("test_input.txt").expect("Test file should exist");

        let expected = 45000;
        assert_eq!(
            solve_part_two_with_result(&file_contents).unwrap(),
            expected
        );
    }
}
