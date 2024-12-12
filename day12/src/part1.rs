use std::cell::Cell;

struct Plot {
    plant: u8,
    visited: Cell<bool>,
}

#[derive(Default)]
struct Region {
    area: usize,
    perimeter: usize,
}

pub fn solve(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&plant| Plot {
                    plant,
                    visited: Cell::new(false),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    map.iter()
        .enumerate()
        .flat_map(|(row, data)| {
            data.iter()
                .enumerate()
                .filter_map(move |(col, plot)| {
                    if !plot.visited.get() {
                        Some((row, col, plot))
                    } else {
                        None
                    }
                })
                .map(|(row, col, plot)| {
                    plot.visited.set(true);
                    let mut region = Region::default();
                    trace_region(&map, row, col, plot.plant, &mut region);
                    region
                })
        })
        .map(|region| region.area * region.perimeter)
        .sum()
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn trace_region(map: &Vec<Vec<Plot>>, row: usize, col: usize, plant: u8, region: &mut Region) {
    region.area += 1;
    for (dr, dc) in DIRECTIONS {
        let Some(row) = row.checked_add_signed(dr) else {
            region.perimeter += 1;
            continue;
        };
        let Some(col) = col.checked_add_signed(dc) else {
            region.perimeter += 1;
            continue;
        };
        let Some(plot) = map.get(row).and_then(|row| row.get(col)) else {
            region.perimeter += 1;
            continue;
        };
        if plot.plant != plant {
            region.perimeter += 1;
        } else if !plot.visited.get() {
            plot.visited.set(true);
            trace_region(map, row, col, plant, region);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");
    const EXAMPLE3: &str = include_str!("../example3.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 140);
    }

    #[test]
    fn example2() {
        let result = solve(EXAMPLE2);
        assert_eq!(result, 772);
    }

    #[test]
    fn example3() {
        let result = solve(EXAMPLE3);
        assert_eq!(result, 1930);
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
