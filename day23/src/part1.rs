use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing delimiter")]
    MissingDelimiter,
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let graph = read_graph(input)?;

    Ok(graph
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
        .count())
}

fn read_graph(input: &str) -> Result<HashMap<&str, HashSet<&str>>, Error> {
    input.lines().try_fold(
        HashMap::new(),
        |mut graph: HashMap<&str, HashSet<&str>>, line| {
            let (a, b) = line.split_once('-').ok_or(Error::MissingDelimiter)?;
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
            Ok(graph)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 7);
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
