use rayon::prelude::*;

pub fn solve(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let mut towels = towels.split(", ").map(|s| s.as_bytes()).collect::<Vec<_>>();
    towels.sort_by_key(|b| std::cmp::Reverse(b.len()));

    patterns
        .par_lines()
        .map(|pattern| {
            let mut memo = vec![None; pattern.len() + 1];
            memo[0] = Some(1);
            count_arrangements(pattern.as_bytes(), &towels, &mut memo)
        })
        .sum()
}

fn count_arrangements(pattern: &[u8], towels: &[&[u8]], memo: &mut [Option<usize>]) -> usize {
    if let Some(count) = memo[pattern.len()] {
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
        memo[pattern.len()] = Some(count);
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
