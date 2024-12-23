use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> String {
    let graph = read_graph(input);
    let mut nodes: Vec<_> = graph.keys().copied().collect();
    nodes.sort();
    let mut result = vec![];
    let mut scratch = vec![];
    find_largest_fully_connected_subgraph(&nodes, &graph, &mut scratch, &mut result);
    result.join(",")
}

fn read_graph(input: &str) -> HashMap<&str, HashSet<&str>> {
    input.lines().fold(HashMap::new(), |mut graph, line| {
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
        graph
    })
}

fn find_largest_fully_connected_subgraph<'a>(
    nodes: &[&'a str],
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    scratch: &mut Vec<&'a str>,
    result: &mut Vec<&'a str>,
) {
    if scratch.len() > result.len() {
        *result = scratch.clone();
    }
    for (i, &node) in nodes.iter().enumerate() {
        if let Some(others) = graph.get(node) {
            if scratch.iter().all(|node| others.contains(node)) {
                scratch.push(node);
                find_largest_fully_connected_subgraph(&nodes[i + 1..], graph, scratch, result);
                scratch.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, "co,de,ka,ta");
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
