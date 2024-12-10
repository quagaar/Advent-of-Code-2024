use rayon::prelude::*;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    map.iter()
        .enumerate()
        .flat_map(|(row, data)| {
            data.iter().enumerate().filter_map(
                move |(col, &cell)| {
                    if cell == b'0' {
                        Some((row, col))
                    } else {
                        None
                    }
                },
            )
        })
        .par_bridge()
        .map(|(row, col)| {
            let mut peaks = HashSet::new();
            walk_trails(&map, row, col, b'0', &mut peaks);
            peaks.len()
        })
        .sum()
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn walk_trails(
    map: &[&[u8]],
    row: usize,
    col: usize,
    height: u8,
    peaks: &mut HashSet<(usize, usize)>,
) {
    if height == b'9' {
        peaks.insert((row, col));
    } else {
        let next_height = height + 1;
        for (dr, dc) in DIRECTIONS {
            let Some(row) = row.checked_add_signed(dr) else {
                continue;
            };
            let Some(col) = col.checked_add_signed(dc) else {
                continue;
            };
            let Some(data) = map.get(row) else { continue };
            let Some(cell) = data.get(col) else { continue };
            if *cell == next_height {
                walk_trails(map, row, col, next_height, peaks);
            }
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
        assert_eq!(result, 36);
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
