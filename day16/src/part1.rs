use pathfinding::prelude::dijkstra;
use std::iter::once;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    row: usize,
    column: usize,
    direction: Direction,
}

pub fn solve(input: &str) -> usize {
    let maze = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let Some((start, end)) = find_start_and_end(&maze) else {
        panic!("Start or end not found");
    };

    dijkstra(
        &start,
        |node| get_successors(node, &maze),
        |node| node.row == end.0 && node.column == end.1,
    )
    .expect("No path to end found")
    .1
}

fn find_start_and_end(maze: &[&[u8]]) -> Option<(Node, (usize, usize))> {
    let mut start = None;
    let mut end = None;
    for (row, line) in maze.iter().enumerate() {
        for (column, &cell) in line.iter().enumerate() {
            if cell == b'S' {
                start = Some(Node {
                    row,
                    column,
                    direction: Direction::East,
                });
            }
            if cell == b'E' {
                end = Some((row, column));
            }
        }
    }
    Some((start?, end?))
}

fn get_successors(node: &Node, maze: &[&[u8]]) -> impl Iterator<Item = (Node, usize)> {
    go_forward(node, maze)
        .map(|node| (node, 1))
        .into_iter()
        .chain(once((turn_right(node), 1000)))
        .chain(once((turn_left(node), 1000)))
}

fn go_forward(node: &Node, maze: &[&[u8]]) -> Option<Node> {
    let (row, column) = match node.direction {
        Direction::North => (node.row.checked_sub(1)?, node.column),
        Direction::East => (node.row, node.column.checked_add(1)?),
        Direction::South => (node.row.checked_add(1)?, node.column),
        Direction::West => (node.row, node.column.checked_sub(1)?),
    };
    if *maze.get(row)?.get(column)? == b'#' {
        None
    } else {
        Some(Node {
            row,
            column,
            ..*node
        })
    }
}

fn turn_right(node: &Node) -> Node {
    Node {
        direction: match node.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        },
        ..*node
    }
}

fn turn_left(node: &Node) -> Node {
    Node {
        direction: match node.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        },
        ..*node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 7036);
    }

    #[test]
    fn example2() {
        let result = solve(EXAMPLE2);
        assert_eq!(result, 11048);
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
