use rayon::prelude::*;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let (map, start) = parse_input(input);
    let mut visited = HashSet::new();
    let mut guard = start;

    visited.insert(guard.location);

    while let Some(next) = guard.next(&map) {
        guard = next;
        visited.insert(guard.location);
    }

    visited
        .into_par_iter()
        .filter(|loc| *loc != start.location)
        .filter(|loc| guard_will_loop(&start, &map, loc))
        .count()
}

fn parse_input(input: &str) -> (Map, Guard) {
    let mut obstructions = HashSet::new();
    let mut width = 0;
    let mut height = 0;
    let mut start = Location { row: 0, column: 0 };
    for (row, line) in input.lines().enumerate() {
        height += 1;
        width = width.max(line.len() as i16);
        for (column, c) in line.chars().enumerate() {
            if c == '#' {
                obstructions.insert(Location {
                    row: row as i16,
                    column: column as i16,
                });
            }
            if c == '^' {
                start = Location {
                    row: row as i16,
                    column: column as i16,
                };
            }
        }
    }
    (
        Map {
            width,
            height,
            obstructions,
        },
        Guard {
            location: start,
            direction: Direction::North,
        },
    )
}

fn guard_will_loop(start: &Guard, map: &Map, new_obstruction: &Location) -> bool {
    let mut guard = *start;
    let mut map = map.clone();
    map.obstructions.insert(*new_obstruction);

    let mut visited = HashSet::new();

    while visited.insert(guard) {
        match guard.next(&map) {
            Some(next) => guard = next,
            None => return false,
        }
    }

    true
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Location {
    row: i16,
    column: i16,
}

#[derive(Clone)]
struct Map {
    width: i16,
    height: i16,
    obstructions: HashSet<Location>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Guard {
    location: Location,
    direction: Direction,
}

impl Guard {
    fn next(&self, map: &Map) -> Option<Self> {
        let mut next = *self;
        match next.direction {
            Direction::North => next.location.row -= 1,
            Direction::East => next.location.column += 1,
            Direction::South => next.location.row += 1,
            Direction::West => next.location.column -= 1,
        }
        if (0..map.height).contains(&next.location.row)
            && (0..map.width).contains(&next.location.column)
        {
            if map.obstructions.contains(&next.location) {
                next.direction = match next.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
                next.location = self.location;
            }
            Some(next)
        } else {
            None
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
        assert_eq!(result, 6);
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
