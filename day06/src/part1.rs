use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let (map, mut guard) = parse_input(input);
    let mut visited = HashSet::new();

    visited.insert(guard.location);

    loop {
        match guard.next(&map) {
            Some(next) => {
                guard = next;
                visited.insert(guard.location);
            }
            None => break visited.len(),
        }
    }
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Location {
    row: i16,
    column: i16,
}

struct Map {
    width: i16,
    height: i16,
    obstructions: HashSet<Location>,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
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
        assert_eq!(result, 41);
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
