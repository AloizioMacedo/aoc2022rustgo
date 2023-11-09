use day7::parsing::{build_file_system, ExecutionError};

const INPUT: &str = include_str!("../input.txt");

fn solve_part_one(contents: &str) -> Result<usize, ExecutionError> {
    let fs = build_file_system(contents)?;

    Ok(fs
        .get_dir_sizes()
        .into_iter()
        .filter(|&size| size <= 100000)
        .sum())
}

fn solve_part_two(contents: &str) -> Result<usize, ExecutionError> {
    let fs = build_file_system(contents)?;

    const TOTAL_DISK: usize = 70_000_000;
    const REQUIRED_UNUSED: usize = 30_000_000;

    let sizes = fs.get_dir_sizes();
    let root_size = sizes[0];

    let unused = TOTAL_DISK - root_size;
    let needed = REQUIRED_UNUSED.saturating_sub(unused);

    Ok(*sizes
        .iter()
        .filter(|&&size| size >= needed)
        .min()
        .expect("File system has at least root"))
}

fn main() -> Result<(), ExecutionError> {
    println!("{}", solve_part_one(INPUT)?);
    println!("{:?}", solve_part_two(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use day7::parsing::build_file_system;

    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_size() {
        let fs = build_file_system(TEST_INPUT).unwrap();

        let dir_sizes = fs.get_dir_sizes();
        assert_eq!(dir_sizes[0], 48381165);
        assert_eq!(dir_sizes[1], 94853);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT).unwrap(), 95437);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT).unwrap(), 24933642);
    }
}
