use std::collections::HashMap;

use petgraph::{algo::dijkstra, prelude::UnGraphMap};
use thiserror::Error;

const INPUT: &str = include_str!("../input.txt");

fn get_optimum_pressure_release(graph: &UnGraphMap<&str, u64>, flows: &HashMap<&str, u64>) -> u64 {
    recursive_max(
        graph,
        flows,
        0,
        0,
        "AA",
        &graph
            .nodes()
            .filter(|&x| x != "AA" && flows[x] != 0)
            .collect::<Vec<&str>>()[..],
        0,
    )
}

fn recursive_max(
    graph: &UnGraphMap<&str, u64>,
    flows: &HashMap<&str, u64>,
    time: u64,
    current_pressure: u64,
    current_node: &str,
    nodes_left: &[&str],
    current_max: u64,
) -> u64 {
    let mut current_values = Vec::new();

    for node in nodes_left {
        let time_to_node = graph
            .edge_weight(current_node, node)
            .expect("Graph should be complete");

        let time_after_opening_valve_in_destination = time + time_to_node + 1;
        if time_after_opening_valve_in_destination > 30 {
            current_values.push(current_pressure)
        } else {
            current_values.push(recursive_max(
                graph,
                flows,
                time_after_opening_valve_in_destination,
                current_pressure + flows[node] * (30 - time_after_opening_valve_in_destination),
                node,
                nodes_left
                    .iter()
                    .copied()
                    .filter(|x| x != node)
                    .collect::<Vec<&str>>()
                    .as_slice(),
                current_max,
            ))
        }
    }

    if nodes_left.is_empty() {
        return current_pressure;
    }

    *current_values.iter().max().unwrap_or(&0).max(&current_max)
}

fn get_optimum_pressure_release_with_elephant(
    graph: &UnGraphMap<&str, u64>,
    flows: &HashMap<&str, u64>,
) -> u64 {
    recursive_max_with_elephant2(
        graph,
        flows,
        0,
        0,
        0,
        "AA",
        "AA",
        &graph
            .nodes()
            .filter(|&x| x != "AA" && flows[x] != 0)
            .collect::<Vec<&str>>()[..],
        0,
        true,
    )
}

fn recursive_max_with_elephant2(
    graph: &UnGraphMap<&str, u64>,
    flows: &HashMap<&str, u64>,
    time_me: u64,
    time_elephant: u64,
    current_pressure: u64,
    current_node: &str,
    current_node_elephant: &str,
    nodes_left: &[&str],
    current_max: u64,
    is_me: bool,
) -> u64 {
    let mut current_values = Vec::new();

    for node in nodes_left {
        let time_to_node = graph
            .edge_weight(current_node, node)
            .expect("Graph should be complete");

        let time_to_elephant_node = graph
            .edge_weight(current_node_elephant, node)
            .expect("Graph should be complete");

        if is_me {
            let time_after_opening_valve_in_destination = time_me + time_to_node + 1;
            if time_after_opening_valve_in_destination > 26 {
                let time_after_elephant_opening_valve_in_destination =
                    time_elephant + time_to_elephant_node + 1;

                if time_after_elephant_opening_valve_in_destination > 26 {
                    current_values.push(current_pressure)
                } else {
                    current_values.push(recursive_max_with_elephant2(
                        graph,
                        flows,
                        time_me,
                        time_after_elephant_opening_valve_in_destination,
                        current_pressure
                            + flows[node] * (26 - time_after_elephant_opening_valve_in_destination),
                        current_node,
                        node,
                        nodes_left
                            .iter()
                            .copied()
                            .filter(|x| x != node)
                            .collect::<Vec<&str>>()
                            .as_slice(),
                        current_max,
                        false,
                    ))
                }
            } else {
                current_values.push(recursive_max_with_elephant2(
                    graph,
                    flows,
                    time_after_opening_valve_in_destination,
                    time_elephant,
                    current_pressure + flows[node] * (26 - time_after_opening_valve_in_destination),
                    node,
                    current_node_elephant,
                    nodes_left
                        .iter()
                        .copied()
                        .filter(|x| x != node)
                        .collect::<Vec<&str>>()
                        .as_slice(),
                    current_max,
                    false,
                ))
            }
        } else {
            let time_after_elephant_opening_valve_in_destination =
                time_elephant + time_to_elephant_node + 1;
            if time_after_elephant_opening_valve_in_destination > 26 {
                let time_to_node_me = graph
                    .edge_weight(current_node, node)
                    .expect("Graph should be complete");

                let time_after_opening_valve_in_destination = time_me + time_to_node_me + 1;

                if time_after_opening_valve_in_destination > 26 {
                    current_values.push(current_pressure)
                } else {
                    current_values.push(recursive_max_with_elephant2(
                        graph,
                        flows,
                        time_after_opening_valve_in_destination,
                        time_elephant,
                        current_pressure
                            + flows[node] * (26 - time_after_opening_valve_in_destination),
                        node,
                        current_node_elephant,
                        nodes_left
                            .iter()
                            .copied()
                            .filter(|x| x != node)
                            .collect::<Vec<&str>>()
                            .as_slice(),
                        current_max,
                        true,
                    ))
                }
            } else {
                current_values.push(recursive_max_with_elephant2(
                    graph,
                    flows,
                    time_me,
                    time_after_elephant_opening_valve_in_destination,
                    current_pressure
                        + flows[node] * (26 - time_after_elephant_opening_valve_in_destination),
                    current_node,
                    node,
                    nodes_left
                        .iter()
                        .copied()
                        .filter(|x| x != node)
                        .collect::<Vec<&str>>()
                        .as_slice(),
                    current_max,
                    true,
                ))
            }
        }
    }

    if nodes_left.is_empty() {
        return current_pressure;
    }

    *current_values.iter().max().unwrap_or(&0).max(&current_max)
}

fn get_valve_graph(
    contents: &str,
) -> Result<(UnGraphMap<&str, u64>, HashMap<&str, u64>), ParseError> {
    let mut graph = UnGraphMap::<&str, u64>::new();
    let mut flows = HashMap::new();

    contents.lines().try_for_each(|line| {
        if let Ok(parsed_line) = parse_line(line) {
            for target_valve in parsed_line.1 {
                graph.add_edge(parsed_line.0, target_valve, 1);
            }

            flows.entry(parsed_line.0).or_insert(parsed_line.2);

            Ok(())
        } else {
            Err(ParseError(line.to_string()))
        }
    })?;

    let mut final_graph = graph.clone();

    for node in graph.nodes() {
        let distances = dijkstra(&graph, node, None, |_| 1);
        for other_node in graph.nodes().filter(|x| *x != node) {
            final_graph.add_edge(node, other_node, distances[other_node]);
        }
    }

    Ok((final_graph, flows))
}

#[derive(Debug, Error)]
#[error("Parse error")]
struct ParseError(String);

fn parse_line(line: &str) -> Result<(&str, Vec<&str>, u64), ParseError> {
    match &line
        .split(&[' ', '=', ';', ','])
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()[..]
    {
        [_, origin, _has, _flow, _rate, rate, _tunnels, _lead, _to, _valves, valves @ ..] => Ok((
            origin,
            valves.to_vec(),
            rate.parse().map_err(|_| ParseError(line.to_string()))?,
        )),
        _ => Err(ParseError(line.to_string())),
    }
}

fn solve_part_one(contents: &str) -> Result<u64, ParseError> {
    let (graph, flows) = get_valve_graph(contents)?;

    Ok(get_optimum_pressure_release(&graph, &flows))
}

fn solve_part_two(contents: &str) -> Result<u64, ParseError> {
    let (graph, flows) = get_valve_graph(contents)?;

    Ok(get_optimum_pressure_release_with_elephant(&graph, &flows))
}

fn main() -> Result<(), ParseError> {
    println!("{}", solve_part_one(INPUT)?);

    // Takes some time to run.
    println!("{}", solve_part_two(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = include_str!("../test_input.txt");

    #[test]
    fn it_works() {
        let parsed_line1 =
            parse_line("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB").unwrap();
        let parsed_line2 =
            parse_line("Valve CC has flow rate=2; tunnels lead to valves DD, BB").unwrap();

        assert_eq!(parsed_line1, ("AA", vec!["DD", "II", "BB"], 0));
        assert_eq!(parsed_line2, ("CC", vec!["DD", "BB"], 2));
    }

    #[test]
    fn part_one() {
        let (graph, flows) = get_valve_graph(TEST).unwrap();

        assert_eq!(get_optimum_pressure_release(&graph, &flows), 1651);
    }

    #[test]
    fn part_two() {
        let (graph, flows) = get_valve_graph(TEST).unwrap();

        assert_eq!(
            get_optimum_pressure_release_with_elephant(&graph, &flows),
            1707
        );
    }
}
