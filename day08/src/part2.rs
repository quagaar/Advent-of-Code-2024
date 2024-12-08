use itertools::{chain, Itertools};
use num_integer::Integer;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
    let map = Map::parse(input);

    map.antennas
        .iter()
        .flat_map(|(_, locations)| {
            locations
                .iter()
                .tuple_combinations()
                .flat_map(|(a, b)| antinodes(a, b, &map))
        })
        .collect::<HashSet<Location>>()
        .len()
}

fn antinodes<'a>(
    a: &'a Location,
    b: &'a Location,
    map: &'a Map,
) -> impl Iterator<Item = Location> + 'a {
    let delta_row = a.row - b.row;
    let delta_column = a.column - b.column;
    let gcd = delta_row.gcd(&delta_column);
    let delta_row = delta_row / gcd;
    let delta_column = delta_column / gcd;

    chain![
        between_antinodes(a, b, delta_row, delta_column),
        node_antinodes(a, map, delta_row, delta_column),
        node_antinodes(b, map, -delta_row, -delta_column),
    ]
}

fn between_antinodes<'a>(
    a: &'a Location,
    b: &'a Location,
    delta_row: i16,
    delta_column: i16,
) -> impl Iterator<Item = Location> + 'a {
    std::iter::successors(Some(*a), move |node| {
        Some(Location {
            row: node.row - delta_row,
            column: node.column - delta_column,
        })
    })
    .skip(1)
    .take_while(move |location| location != b)
}

fn node_antinodes<'a>(
    node: &'a Location,
    map: &'a Map,
    delta_row: i16,
    delta_column: i16,
) -> impl Iterator<Item = Location> + 'a {
    std::iter::successors(Some(*node), move |node| {
        Some(Location {
            row: node.row + delta_row,
            column: node.column + delta_column,
        })
    })
    .take_while(|location| map.contains(*location))
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Location {
    row: i16,
    column: i16,
}

struct Map {
    width: i16,
    height: i16,
    antennas: HashMap<char, Vec<Location>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut antennas: HashMap<char, Vec<Location>> = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        for (row, line) in input.lines().enumerate() {
            height += 1;
            width = width.max(line.len() as i16);
            for (column, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_default().push(Location {
                        row: row as i16,
                        column: column as i16,
                    });
                }
            }
        }
        Self {
            width,
            height,
            antennas,
        }
    }

    fn contains(&self, location: Location) -> bool {
        location.row >= 0
            && location.row < self.height
            && location.column >= 0
            && location.column < self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 34);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
