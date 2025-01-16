use itertools::Itertools;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<u64, Error> {
    let (left, right) = input.split_whitespace().tuples().try_fold(
        (Vec::with_capacity(1000), HashMap::with_capacity(1000)),
        |(mut left, mut right), (lhs, rhs)| {
            left.push(lhs.parse::<u64>()?);
            right
                .entry(rhs.parse::<u64>()?)
                .and_modify(|x| *x += 1)
                .or_insert(1);
            Ok::<_, Error>((left, right))
        },
    )?;
    Ok(left
        .into_iter()
        .map(|lhs| lhs * right.get(&lhs).unwrap_or(&0))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 31);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
