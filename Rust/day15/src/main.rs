use std::num::ParseIntError;

use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");
const LOWER_BOUND: i32 = -10_000_000;
const UPPER_BOUND: i32 = 10_000_000;

#[derive(Debug)]
struct Sensor(i32, i32);

#[derive(Debug)]
struct Beacon(i32, i32);

struct SensorWithRadiusAndBeacon {
    sensor: Sensor,
    beacon: Beacon,
    radius: u32,
}

fn build_sensors_with_radius(contents: &str) -> Result<Vec<SensorWithRadiusAndBeacon>, ParseError> {
    let sensors_and_beacons = build_sensors_and_beacons(contents)?;

    Ok(sensors_and_beacons
        .into_iter()
        .map(|(s, b)| SensorWithRadiusAndBeacon {
            radius: get_manhattan_distance(&s, &(b.0, b.1)),
            sensor: s,
            beacon: b,
        })
        .collect())
}

fn build_sensors_and_beacons(contents: &str) -> Result<Vec<(Sensor, Beacon)>, ParseError> {
    contents.lines().map(parse_line).collect()
}

fn get_manhattan_distance(sensor: &Sensor, pos: &(i32, i32)) -> u32 {
    (sensor.0 - pos.0).unsigned_abs() + (sensor.1 - pos.1).unsigned_abs()
}

#[derive(Error, Debug)]
#[error("Parse error")]
enum ParseError {
    ParseIntError(#[from] ParseIntError),
    SyntaxError(String),
}

fn parse_line(line: &str) -> Result<(Sensor, Beacon), ParseError> {
    let split_line: Vec<&str> = line
        .split(&[',', ' ', '=', ':'])
        .filter(|s| !s.is_empty())
        .collect();

    #[allow(unused_variables)]
    match split_line[..] {
        [_sensor, _ats, _xs, x_sensor, _ys, y_sensor, _closest, _beacon, _is, _atb, _x_b, x_beacon, _y_b, y_beacon] => {
            Ok((
                Sensor(x_sensor.parse()?, y_sensor.parse()?),
                Beacon(x_beacon.parse()?, y_beacon.parse()?),
            ))
        }
        _ => Err(ParseError::SyntaxError(line.to_string())),
    }
}

fn solve_part_one(contents: &str, y: i32) -> Result<usize, ParseError> {
    let sensors_with_radius = build_sensors_with_radius(contents)?;

    Ok((LOWER_BOUND..UPPER_BOUND)
        .map(|i| (i, y))
        .filter(|pos| {
            sensors_with_radius.iter().any(|sensor| {
                get_manhattan_distance(&sensor.sensor, pos) <= sensor.radius
                    && (sensor.beacon.0, sensor.beacon.1) != *pos
            })
        })
        .count())
}

fn main() -> Result<(), ParseError> {
    println!("{}", solve_part_one(INPUT, 2000000)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = include_str!("../test_input.txt");

    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(TEST, 10).unwrap(), 26);
    }
}
