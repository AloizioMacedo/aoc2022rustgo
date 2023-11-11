use serde_json::json;
use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

struct Pair<'a> {
    first: &'a str,
    second: &'a str,
}

fn get_pairs(contents: &str) -> Vec<Pair> {
    let lines: Vec<&str> = contents.lines().filter(|line| !line.is_empty()).collect();

    lines
        .chunks_exact(2)
        .map(|chunk| {
            let first = chunk[0];
            let second = chunk[1];

            Pair { first, second }
        })
        .collect()
}

#[derive(Error, Debug)]
#[error("Parsing error")]
struct ParsingError(String);

fn is_valid(pair: &Pair) -> Result<bool, ParsingError> {
    let first = pair.first;
    let second = pair.second;

    let json_first: serde_json::Value =
        serde_json::from_str(first).map_err(|_| ParsingError(first.to_string()))?;

    let json_second: serde_json::Value =
        serde_json::from_str(second).map_err(|_| ParsingError(second.to_string()))?;

    match is_valid_json_pair(&json_first, &json_second) {
        Ok(EvaluationResult::Valid) => Ok(true),
        _ => Ok(false),
    }
}

enum EvaluationResult {
    Valid,
    Inconclusive,
    Invalid,
}

fn is_valid_json_pair(
    json1: &serde_json::Value,
    json2: &serde_json::Value,
) -> Result<EvaluationResult, ParsingError> {
    match (json1, json2) {
        (serde_json::Value::Number(n1), serde_json::Value::Number(n2)) => {
            let n1 = n1.as_i64();
            let n2 = n2.as_i64();

            match n1.cmp(&n2) {
                std::cmp::Ordering::Less => Ok(EvaluationResult::Valid),
                std::cmp::Ordering::Equal => Ok(EvaluationResult::Inconclusive),
                std::cmp::Ordering::Greater => Ok(EvaluationResult::Invalid),
            }
        }
        (serde_json::Value::Array(a1), serde_json::Value::Array(a2)) => {
            for (i1, i2) in a1.iter().zip(a2.iter()) {
                let result = is_valid_json_pair(i1, i2);

                match result {
                    Ok(EvaluationResult::Valid) => return Ok(EvaluationResult::Valid),
                    Ok(EvaluationResult::Inconclusive) => continue,
                    Ok(EvaluationResult::Invalid) => return Ok(EvaluationResult::Invalid),
                    _ => return Err(ParsingError(json1.to_string() + &json2.to_string())),
                }
            }

            match a1.len().cmp(&a2.len()) {
                std::cmp::Ordering::Less => Ok(EvaluationResult::Valid),
                std::cmp::Ordering::Equal => Ok(EvaluationResult::Inconclusive),
                std::cmp::Ordering::Greater => Ok(EvaluationResult::Invalid),
            }
        }
        (serde_json::Value::Number(n), serde_json::Value::Array(a)) => {
            is_valid_json_pair(&json!([n]), &json!(a))
        }
        (serde_json::Value::Array(a), serde_json::Value::Number(n)) => {
            is_valid_json_pair(&json!(a), &json!([n]))
        }
        _ => Err(ParsingError(json1.to_string() + &json2.to_string())),
    }
}

fn solve_part_one(contents: &str) -> Result<usize, ParsingError> {
    let pairs = get_pairs(contents);

    let mut total = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if is_valid(pair)? {
            total += i + 1;
        }
    }

    Ok(total)
}

fn main() -> Result<(), ParsingError> {
    println!("{}", solve_part_one(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = include_str!("../test_input.txt");

    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(TEST).unwrap(), 13)
    }
}
