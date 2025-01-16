use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<u64, Error> {
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = input
        .split_whitespace()
        .tuples()
        .map(|(lhs, rhs)| Ok((lhs.parse::<u64>()?, rhs.parse::<u64>()?)))
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()?
        .into_iter()
        .unzip();
    left.sort();
    right.sort();
    Ok(left
        .into_iter()
        .zip(right)
        .map(|(lhs, rhs)| lhs.abs_diff(rhs))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 11);
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
