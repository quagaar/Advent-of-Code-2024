use regex::Regex;
use std::sync::OnceLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing regex: {0}")]
    RegexError(#[from] &'static regex::Error),
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

static RE: OnceLock<Result<Regex, regex::Error>> = OnceLock::new();

pub fn solve(input: &str) -> Result<usize, Error> {
    let rex = RE
        .get_or_init(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)"))
        .as_ref()?;
    rex.captures_iter(input).try_fold(0, |acc, c| {
        let lhs = c[1].parse::<usize>()?;
        let rhs = c[2].parse::<usize>()?;
        Ok(acc + lhs * rhs)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 161);
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
