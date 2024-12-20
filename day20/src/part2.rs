use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    shortcuts(input)
        .into_iter()
        .filter(|&(_, t)| t >= 100)
        .count()
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

type Position = (usize, usize);
type ShortcutMap = HashMap<(Position, Position), usize>;

fn shortcuts(input: &str) -> ShortcutMap {
    let map = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let (start, end) = find_start_and_end(&map);
    let (route, _length) = dijkstra(
        &start,
        |&(x, y)| successors(x, y, &map),
        |position| *position == end,
    )
    .expect("no path found");

    let route_map = route
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| ((x, y), i))
        .collect::<HashMap<_, _>>();

    route
        .into_iter()
        .enumerate()
        .flat_map(|(offset, position)| shortcuts_from_position(position, offset, &route_map, &map))
        .collect()
}

fn find_start_and_end(map: &[&[u8]]) -> (Position, Position) {
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
) -> impl Iterator<Item = (Position, usize)> + 'a {
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
    position: Position,
    offset: usize,
    route_map: &'a HashMap<Position, usize>,
    map: &'a [&[u8]],
) -> impl Iterator<Item = ((Position, Position), usize)> + 'a {
    shortcut_positions(position).filter_map(move |((x, y), shortcut_length)| {
        let cell = map.get(y)?.get(x)?;
        if *cell == b'#' {
            return None;
        }
        let next = route_map.get(&(x, y))?;
        if *next <= offset + shortcut_length {
            None
        } else {
            Some(((position, (x, y)), next - offset - shortcut_length))
        }
    })
}

fn shortcut_positions(start: Position) -> impl Iterator<Item = (Position, usize)> {
    (1..20)
        .flat_map(move |n| {
            (1..=20 - n).flat_map(move |m| {
                [(n, m), (-n, m), (-n, -m), (n, -m)]
                    .into_iter()
                    .filter_map(move |(dx, dy)| {
                        let x = start.0.checked_add_signed(dx)?;
                        let y = start.1.checked_add_signed(dy)?;
                        Some(((x, y), (n + m) as usize))
                    })
            })
        })
        .chain((1..=20).flat_map(move |n| {
            [(n, 0), (0, n), (-n, 0), (0, -n)]
                .into_iter()
                .filter_map(move |(dx, dy)| {
                    let x = start.0.checked_add_signed(dx)?;
                    let y = start.1.checked_add_signed(dy)?;
                    Some(((x, y), n as usize))
                })
        }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = shortcuts(EXAMPLE);
        println!("{:?}", result);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 50).count(), 32);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 52).count(), 31);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 54).count(), 29);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 56).count(), 39);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 58).count(), 25);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 60).count(), 23);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 62).count(), 20);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 64).count(), 19);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 66).count(), 12);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 68).count(), 14);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 70).count(), 12);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 72).count(), 22);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 74).count(), 4);
        assert_eq!(result.iter().filter(|&(_, &t)| t == 76).count(), 3);
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
