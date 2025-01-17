use rayon::prelude::*;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Empty input")]
    EmptyInput,
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Unable to parse number, reason: {0}")]
    UnableToParseNumber(#[from] ParseIntError),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    input
        .par_lines()
        .map(process_line)
        .try_reduce(
            || None,
            |a, b| match (a, b) {
                (Some(a), Some(b)) => Ok(Some(a + b)),
                (Some(a), None) => Ok(Some(a)),
                (None, Some(b)) => Ok(Some(b)),
                (None, None) => Ok(None),
            },
        )
        .map(|x| x.unwrap_or(0))
}

fn process_line(line: &str) -> Result<Option<usize>, Error> {
    let (test_value, numbers) = line.split_once(": ").ok_or(Error::MissingDelimiter)?;
    let test_value = test_value.parse::<usize>()?;
    let numbers = numbers
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    if is_valid(test_value, numbers[0], &numbers[1..]) {
        Ok(Some(test_value))
    } else {
        Ok(None)
    }
}

fn is_valid(target: usize, calc: usize, numbers: &[usize]) -> bool {
    if numbers.is_empty() {
        calc == target
    } else if calc > target {
        false
    } else {
        is_valid(target, calc * numbers[0], &numbers[1..])
            || is_valid(target, calc + numbers[0], &numbers[1..])
            || is_valid(target, concat_numbers(calc, numbers[0]), &numbers[1..])
    }
}

fn concat_numbers(lhs: usize, rhs: usize) -> usize {
    if rhs < 10 {
        lhs * 10 + rhs
    } else if rhs < 100 {
        lhs * 100 + rhs
    } else if rhs < 1000 {
        lhs * 1000 + rhs
    } else {
        lhs * 10_usize.pow(rhs.ilog10() + 1) + rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 11387);
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
