use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
    let graph = read_graph(input);
    let mut nodes: Vec<_> = graph.keys().copied().collect();
    nodes.sort();
    let mut scratch = vec![];
    count_interconnected(&nodes, &graph, &mut scratch, 3)
}

fn read_graph(input: &str) -> HashMap<&str, HashSet<&str>> {
    input.lines().fold(HashMap::new(), |mut graph, line| {
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
        graph
    })
}

fn count_interconnected<'a>(
    nodes: &[&'a str],
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    scratch: &mut Vec<&'a str>,
    depth: usize,
) -> usize {
    if depth == 0 {
        if scratch.iter().any(|node| node.starts_with("t")) {
            return 1;
        } else {
            return 0;
        }
    }
    nodes
        .iter()
        .enumerate()
        .filter_map(|(i, &node)| {
            if let Some(others) = graph.get(node) {
                if scratch.iter().all(|node| others.contains(node)) {
                    scratch.push(node);
                    let count = count_interconnected(&nodes[i + 1..], graph, scratch, depth - 1);
                    scratch.pop();
                    Some(count)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 7);
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
