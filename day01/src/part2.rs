use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> u64 {
    let (left, right) = input
        .split_whitespace()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap()))
        .fold(
            (Vec::with_capacity(1000), HashMap::with_capacity(1000)),
            |(mut left, mut right), (lhs, rhs)| {
                left.push(lhs.parse::<u64>().unwrap());
                right
                    .entry(rhs.parse::<u64>().unwrap())
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
                (left, right)
            },
        );
    left.into_iter()
        .map(|lhs| lhs * right.get(&lhs).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 31);
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
