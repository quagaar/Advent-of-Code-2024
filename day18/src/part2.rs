use pathfinding::prelude::dijkstra;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Failed to parse number: {0}")]
    FailedToParseNumber(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    let (x, y) = fist_blocker(input, 71, 71)?;
    Ok(format!("{},{}", x, y))
}

struct GridSpace {
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl GridSpace {
    fn contains(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn fist_blocker(input: &str, width: usize, height: usize) -> Result<(usize, usize), Error> {
    let corrupted = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').ok_or(Error::MissingDelimiter)?;
            Ok((x.parse::<usize>()?, y.parse::<usize>()?))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let grid_space = GridSpace {
        width,
        height,
        start: (0, 0),
        end: (width - 1, height - 1),
    };

    let mut low = 0;
    let mut high = corrupted.len() - 1;

    while low < high {
        let mid = (low + high) / 2;
        if is_blocked(&corrupted[..=mid], &grid_space) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    Ok(corrupted[high])
}

fn is_blocked(corrupted: &[(usize, usize)], grid_space: &GridSpace) -> bool {
    let corrupted = corrupted.iter().copied().collect::<HashSet<_>>();

    dijkstra(
        &grid_space.start,
        |&(x, y)| {
            DIRECTIONS
                .into_iter()
                .map(move |(dx, dy)| (x, y, dx, dy))
                .filter_map(|(x, y, dx, dy)| {
                    let x = x.checked_add_signed(dx)?;
                    let y = y.checked_add_signed(dy)?;
                    if grid_space.contains(x, y) && !corrupted.contains(&(x, y)) {
                        Some(((x, y), 1))
                    } else {
                        None
                    }
                })
        },
        |&position| position == grid_space.end,
    )
    .is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = fist_blocker(EXAMPLE, 7, 7).unwrap();
        assert_eq!(result, (6, 1));
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
