use std::{cell::Cell, collections::HashSet};

struct Plot {
    plant: u8,
    visited: Cell<bool>,
}

#[derive(Default)]
struct Region {
    area: usize,
    sides: usize,
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
                    trace_region(&map, row, col, plot.plant)
                })
        })
        .map(|region| region.area * region.sides)
        .sum()
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn trace_region(map: &[Vec<Plot>], row: usize, col: usize, plant: u8) -> Region {
    let mut region_plots = HashSet::new();
    let mut stack = vec![(row, col)];
    let mut max_row = row;
    let mut max_col = col;
    let mut min_row = row;
    let mut min_col = col;

    while let Some((row, col)) = stack.pop() {
        region_plots.insert((row, col));
        max_row = max_row.max(row);
        max_col = max_col.max(col);
        min_row = min_row.min(row);
        min_col = min_col.min(col);
        for (dr, dc) in DIRECTIONS {
            let Some(row) = row.checked_add_signed(dr) else {
                continue;
            };
            let Some(col) = col.checked_add_signed(dc) else {
                continue;
            };
            let Some(plot) = map.get(row).and_then(|row| row.get(col)) else {
                continue;
            };
            if plot.plant == plant && !plot.visited.get() {
                plot.visited.set(true);
                stack.push((row, col));
            }
        }
    }

    let mut region = Region {
        area: region_plots.len(),
        sides: 0,
    };

    for row in min_row..=max_row {
        let mut top_side = false;
        let mut bottom_side = false;
        for col in min_col..=max_col {
            if region_plots.contains(&(row, col)) {
                if row > 0 && region_plots.contains(&(row - 1, col)) {
                    top_side = false;
                } else if !top_side {
                    region.sides += 1;
                    top_side = true;
                }
                if region_plots.contains(&(row + 1, col)) {
                    bottom_side = false;
                } else if !bottom_side {
                    region.sides += 1;
                    bottom_side = true;
                }
            } else {
                top_side = false;
                bottom_side = false;
            }
        }
    }

    for col in min_col..=max_col {
        let mut left_side = false;
        let mut right_side = false;
        for row in min_row..=max_row {
            if region_plots.contains(&(row, col)) {
                if col > 0 && region_plots.contains(&(row, col - 1)) {
                    left_side = false;
                } else if !left_side {
                    region.sides += 1;
                    left_side = true;
                }
                if region_plots.contains(&(row, col + 1)) {
                    right_side = false;
                } else if !right_side {
                    region.sides += 1;
                    right_side = true;
                }
            } else {
                left_side = false;
                right_side = false;
            }
        }
    }

    region
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");
    const EXAMPLE3: &str = include_str!("../example3.txt");
    const EXAMPLE4: &str = include_str!("../example4.txt");
    const EXAMPLE5: &str = include_str!("../example5.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 80);
    }

    #[test]
    fn example2() {
        let result = solve(EXAMPLE2);
        assert_eq!(result, 436);
    }

    #[test]
    fn example4() {
        let result = solve(EXAMPLE4);
        assert_eq!(result, 236);
    }

    #[test]
    fn example5() {
        let result = solve(EXAMPLE5);
        assert_eq!(result, 368);
    }

    #[test]
    fn example3() {
        let result = solve(EXAMPLE3);
        assert_eq!(result, 1206);
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
