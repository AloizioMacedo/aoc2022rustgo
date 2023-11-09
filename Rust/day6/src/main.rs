use std::collections::HashSet;

const BUFFER: &str = include_str!("../input.txt");

fn solve_part_one(buffer: &str) -> Option<usize> {
    buffer
        .as_bytes()
        .windows(4)
        .position(|x| {
            let set: HashSet<&u8> = HashSet::from_iter(x.iter());
            set.len() == 4
        })
        .map(|x| x + 4)
}

fn solve_part_two(buffer: &str) -> Option<usize> {
    buffer
        .as_bytes()
        .windows(14)
        .position(|x| {
            let set: HashSet<&u8> = HashSet::from_iter(x.iter());
            set.len() == 14
        })
        .map(|x| x + 14)
}

fn main() {
    println!("{:?}", solve_part_one(BUFFER));
    println!("{:?}", solve_part_two(BUFFER));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(solve_part_one("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(
            solve_part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
            10
        );
        assert_eq!(
            solve_part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
            11
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            solve_part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(),
            19
        );
        assert_eq!(solve_part_two("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 23);
        assert_eq!(solve_part_two("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 23);
        assert_eq!(
            solve_part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
            29
        );
        assert_eq!(
            solve_part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
            26
        );
    }
}
