use std::cell::Cell;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

struct Position {
    height: u8,
    rating: Cell<Option<u32>>,
}

pub fn solve(input: &str) -> Result<u32, Error> {
    let map = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&height| Position {
                    height,
                    rating: Cell::new(None),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    Ok(map
        .iter()
        .enumerate()
        .flat_map(|(row, data)| {
            data.iter().enumerate().filter_map(move |(col, cell)| {
                if cell.height == b'0' {
                    Some((row, col))
                } else {
                    None
                }
            })
        })
        .map(|(row, col)| walk_trails(&map, row, col, b'0'))
        .sum())
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn walk_trails(map: &Vec<Vec<Position>>, row: usize, col: usize, height: u8) -> u32 {
    if height == b'9' {
        1
    } else {
        let next_height = height + 1;
        DIRECTIONS
            .iter()
            .map(|(dr, dc)| {
                let Some(row) = row.checked_add_signed(*dr) else {
                    return 0;
                };
                let Some(col) = col.checked_add_signed(*dc) else {
                    return 0;
                };
                let Some(position) = map.get(row).and_then(|row| row.get(col)) else {
                    return 0;
                };
                if position.height == next_height {
                    if let Some(n) = position.rating.get() {
                        n
                    } else {
                        let n = walk_trails(map, row, col, next_height);
                        position.rating.set(Some(n));
                        n
                    }
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 81);
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
