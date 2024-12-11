use rayon::prelude::*;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    input
        .par_split_whitespace()
        .map(|s| {
            let mut memo = HashMap::new();
            count_stones(s.parse().unwrap(), 25, &mut memo)
        })
        .sum()
}

fn count_stones(stone: usize, blinks: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if blinks == 0 {
        1
    } else if let Some(count) = memo.get(&(stone, blinks)) {
        *count
    } else if stone == 0 {
        let count = count_stones(1, blinks - 1, memo);
        memo.insert((stone, blinks), count);
        count
    } else if let Some((a, b)) = split_digits(stone) {
        let count = count_stones(a, blinks - 1, memo) + count_stones(b, blinks - 1, memo);
        memo.insert((stone, blinks), count);
        count
    } else {
        let count = count_stones(stone * 2024, blinks - 1, memo);
        memo.insert((stone, blinks), count);
        count
    }
}

fn split_digits(n: usize) -> Option<(usize, usize)> {
    if n < 10 {
        None
    } else if n < 100 {
        Some((n / 10, n % 10))
    } else if n < 1000 {
        None
    } else if n < 10000 {
        Some((n / 100, n % 100))
    } else if n < 100000 {
        None
    } else if n < 1000000 {
        Some((n / 1000, n % 1000))
    } else if n < 10000000 {
        None
    } else if n < 100000000 {
        Some((n / 10000, n % 10000))
    } else if n < 1000000000 {
        None
    } else if n < 10000000000 {
        Some((n / 100000, n % 100000))
    } else if n < 100000000000 {
        None
    } else if n < 1000000000000 {
        Some((n / 1000000, n % 1000000))
    } else if n < 10000000000000 {
        None
    } else if n < 100000000000000 {
        Some((n / 10000000, n % 10000000))
    } else if n < 1000000000000000 {
        None
    } else if n < 10000000000000000 {
        Some((n / 100000000, n % 100000000))
    } else if n < 100000000000000000 {
        None
    } else if n < 1000000000000000000 {
        Some((n / 1000000000, n % 1000000000))
    } else if n < 10000000000000000000 {
        None
    } else {
        Some((n / 10000000000, n % 10000000000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 55312);
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
