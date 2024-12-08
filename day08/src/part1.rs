use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
    let map = Map::parse(input);

    map.antennas
        .iter()
        .flat_map(|(_, locations)| {
            locations
                .iter()
                .tuple_combinations()
                .flat_map(|(a, b)| antinodes(a, b))
        })
        .filter(|location| map.contains(*location))
        .collect::<HashSet<Location>>()
        .len()
}

fn antinodes(a: &Location, b: &Location) -> [Location; 2] {
    let diff_row = a.row - b.row;
    let diff_column = a.column - b.column;
    [
        Location {
            row: a.row + diff_row,
            column: a.column + diff_column,
        },
        Location {
            row: b.row - diff_row,
            column: b.column - diff_column,
        },
    ]
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
        assert_eq!(result, 14);
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
