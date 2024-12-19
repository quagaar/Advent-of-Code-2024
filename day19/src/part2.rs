use rayon::prelude::*;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let mut towels = towels.split(", ").map(|s| s.as_bytes()).collect::<Vec<_>>();
    towels.sort_by_key(|b| std::cmp::Reverse(b.len()));

    patterns
        .par_lines()
        .map(|pattern| {
            let mut memo = HashMap::new();
            memo.insert("".as_bytes(), 1);
            count_arrangements(pattern.as_bytes(), &towels, &mut memo)
        })
        .sum()
}

fn count_arrangements<'a>(
    pattern: &'a [u8],
    towels: &[&[u8]],
    memo: &mut HashMap<&'a [u8], usize>,
) -> usize {
    if let Some(&count) = memo.get(pattern) {
        count
    } else {
        let count = towels
            .iter()
            .map(|towel| {
                pattern
                    .strip_prefix(*towel)
                    .map_or(0, |remaining| count_arrangements(remaining, towels, memo))
            })
            .sum();
        memo.insert(pattern, count);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 16);
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
