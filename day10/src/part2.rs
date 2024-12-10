use rayon::prelude::*;

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
        .map(|(row, col)| walk_trails(&map, row, col, b'0'))
        .sum()
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn walk_trails(map: &[&[u8]], row: usize, col: usize, height: u8) -> usize {
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
                let Some(data) = map.get(row) else { return 0 };
                let Some(cell) = data.get(col) else { return 0 };
                if *cell == next_height {
                    walk_trails(map, row, col, next_height)
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
        let result = solve(EXAMPLE);
        assert_eq!(result, 81);
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
