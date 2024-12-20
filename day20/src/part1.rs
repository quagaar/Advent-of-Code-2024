use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    shortcuts(input)
        .into_iter()
        .filter(|&(_, t)| t >= 100)
        .count()
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

type ShortcutMap = HashMap<((usize, usize), (usize, usize)), usize>;

fn shortcuts(input: &str) -> ShortcutMap {
    let map = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let (start, end) = find_start_and_end(&map);
    let (route, length) = dijkstra(
        &start,
        |&(x, y)| successors(x, y, &map),
        |position| *position == end,
    )
    .expect("no path found");

    (0..route.len() - 1)
        .flat_map(|i| {
            shortcuts_from_position(&route[0..=i], end, &map).filter_map(
                move |(start, end, extra)| {
                    let saving = length.checked_sub(i + extra)?;
                    if saving > 0 {
                        Some(((start, end), saving))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

fn find_start_and_end(map: &[&[u8]]) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == b'S') {
            start = (x, y);
        }
        if let Some(x) = row.iter().position(|&c| c == b'E') {
            end = (x, y);
        }
    }
    (start, end)
}

fn successors<'a>(
    x: usize,
    y: usize,
    map: &'a [&[u8]],
) -> impl Iterator<Item = ((usize, usize), usize)> + 'a {
    DIRECTIONS
        .into_iter()
        .filter_map(move |(dx, dy)| {
            let x = x.checked_add_signed(dx)?;
            let y = y.checked_add_signed(dy)?;
            Some((x, y))
        })
        .filter_map(|(x, y)| {
            let cell = map.get(y)?.get(x)?;
            if *cell == b'#' {
                None
            } else {
                Some(((x, y), 1))
            }
        })
}

fn shortcuts_from_position<'a>(
    route: &'a [(usize, usize)],
    end: (usize, usize),
    map: &'a [&[u8]],
) -> impl Iterator<Item = ((usize, usize), (usize, usize), usize)> + 'a {
    let start = route.last().copied().unwrap();
    DIRECTIONS
        .into_iter()
        .filter_map(move |(dx, dy)| {
            let x = start.0.checked_add_signed(dx)?;
            let y = start.1.checked_add_signed(dy)?;
            let cell = map.get(y)?.get(x)?;
            if *cell == b'#' {
                Some((x, y))
            } else {
                None
            }
        })
        .flat_map(move |(x, y)| {
            DIRECTIONS.into_iter().filter_map(move |(dx, dy)| {
                let x = x.checked_add_signed(dx)?;
                let y = y.checked_add_signed(dy)?;
                let cell = map.get(y)?.get(x)?;
                if *cell == b'#' || route.contains(&(x, y)) {
                    None
                } else {
                    Some((x, y))
                }
            })
        })
        .filter_map(move |new_start| {
            dijkstra(
                &new_start,
                |&(x, y)| successors(x, y, map),
                |position| *position == end,
            )
            .map(|(route, length)| (start, route[0], length + 2))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = shortcuts(EXAMPLE);
        println!("{:?}", result);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 2).count(), 14);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 4).count(), 14);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 6).count(), 2);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 8).count(), 4);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 10).count(), 2);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 12).count(), 3);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 20).count(), 1);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 36).count(), 1);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 38).count(), 1);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 40).count(), 1);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 64).count(), 1);
        assert_eq!(result.len(), 44);
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
