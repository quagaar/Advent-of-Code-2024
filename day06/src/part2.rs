use rayon::prelude::*;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub fn solve(input: &str) -> Result<usize, Error> {
    let (map, start) = parse_input(input);
    let mut visited = HashSet::from([start]);
    let mut history = HashSet::new();
    let mut guard = Guard {
        location: start,
        direction: Direction::North,
    };

    Ok(std::iter::from_fn(|| {
        while let Some(next) = guard.next(&map) {
            history.insert(guard);
            let prev = guard;
            guard = next;
            if visited.insert(guard.location) {
                return Some((history.clone(), prev, guard.location));
            }
        }
        None
    })
    .par_bridge()
    .map(|(history, guard, location)| guard_will_loop(history, guard, &map, location))
    .filter(|x| *x)
    .count())
}

fn parse_input(input: &str) -> (Map, Location) {
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
        start,
    )
}

fn guard_will_loop(
    mut visited: HashSet<Guard>,
    mut guard: Guard,
    map: &Map,
    new_obstruction: Location,
) -> bool {
    let mut map = map.clone();
    map.obstructions.insert(new_obstruction);

    if let Some(next) = guard.next(&map) {
        guard = next;
        while visited.insert(guard) {
            match guard.next(&map) {
                Some(next) => guard = next,
                None => return false,
            }
        }
        true
    } else {
        false
    }
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
        next.move_forward();
        if (0..map.height).contains(&next.location.row)
            && (0..map.width).contains(&next.location.column)
        {
            if map.obstructions.contains(&next.location) {
                next.location = self.location;
                next.turn_right();
            }
            Some(next)
        } else {
            None
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::North => self.location.row -= 1,
            Direction::East => self.location.column += 1,
            Direction::South => self.location.row += 1,
            Direction::West => self.location.column -= 1,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 6);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
