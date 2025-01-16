use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn solve(input: &str) -> Result<usize, Error> {
    let grid = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let cols = 0..grid[0].len();
    let mut xmas_count = 0;

    for start_row in 0..grid.len() {
        for start_col in cols.clone() {
            if grid[start_row][start_col] == b'X' {
                for (diff_row, diff_col) in DIRECTIONS {
                    if check_word(b"MAS", &grid, start_row, start_col, diff_row, diff_col) {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    Ok(xmas_count)
}

fn check_word(
    word: &[u8],
    grid: &[&[u8]],
    start_row: usize,
    start_col: usize,
    diff_row: isize,
    diff_col: isize,
) -> bool {
    let mut rows =
        std::iter::successors(Some(start_row), |r| r.checked_add_signed(diff_row)).skip(1);
    let mut cols =
        std::iter::successors(Some(start_col), |c| c.checked_add_signed(diff_col)).skip(1);

    for word_char in word {
        let Some(row) = rows.next() else {
            return false;
        };
        let Some(col) = cols.next() else {
            return false;
        };
        let Some(row_data) = grid.get(row) else {
            return false;
        };
        let Some(grid_char) = row_data.get(col) else {
            return false;
        };
        if grid_char != word_char {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 18);
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
