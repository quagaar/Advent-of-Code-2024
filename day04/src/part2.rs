pub fn solve(input: &str) -> usize {
    let grid = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let start_cols = 1..grid[0].len() - 1;
    let mut xmas_count = 0;

    for row in 1..grid.len() - 1 {
        for col in start_cols.clone() {
            if grid[row][col] == b'A' {
                match (grid[row - 1][col - 1], grid[row + 1][col + 1]) {
                    (b'M', b'S') | (b'S', b'M') => (),
                    _ => continue,
                }
                match (grid[row - 1][col + 1], grid[row + 1][col - 1]) {
                    (b'M', b'S') | (b'S', b'M') => (),
                    _ => continue,
                }
                xmas_count += 1
            }
        }
    }

    xmas_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 9);
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
