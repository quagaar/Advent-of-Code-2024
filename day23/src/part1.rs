use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
    let graph = read_graph(input);

    graph
        .iter()
        .flat_map(|(node, others)| {
            others
                .iter()
                .tuple_combinations()
                .map(move |(a, b)| (node, a, b))
                .filter(|(node, a, b)| node < a && node < b)
                .filter_map(|(node, a, b)| {
                    if graph.get(a)?.contains(b) {
                        Some((node, a, b))
                    } else {
                        None
                    }
                })
        })
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .count()
}

fn read_graph(input: &str) -> HashMap<&str, HashSet<&str>> {
    input.lines().fold(HashMap::new(), |mut graph, line| {
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
        graph
    })
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
