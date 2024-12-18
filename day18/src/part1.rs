use pathfinding::prelude::dijkstra;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    count_steps(input, 71, 71, 1024)
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn count_steps(input: &str, width: usize, height: usize, bytes: usize) -> usize {
    let corrupted = input
        .lines()
        .take(bytes)
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .unwrap()
        })
        .collect::<HashSet<_>>();
    let start = (0_usize, 0_usize);
    let end = (width - 1, height - 1);

    dijkstra(
        &start,
        |&(x, y)| {
            DIRECTIONS
                .into_iter()
                .map(move |(dx, dy)| (x, y, dx, dy))
                .filter_map(|(x, y, dx, dy)| {
                    let x = x.checked_add_signed(dx)?;
                    let y = y.checked_add_signed(dy)?;
                    if x < width && y < height && !corrupted.contains(&(x, y)) {
                        Some(((x, y), 1))
                    } else {
                        None
                    }
                })
        },
        |&position| position == end,
    )
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = count_steps(EXAMPLE, 7, 7, 12);
        assert_eq!(result, 22);
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
