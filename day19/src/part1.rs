use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing blank line")]
    MissingBlankLine,
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let (towels, patterns) = input.split_once("\n\n").ok_or(Error::MissingBlankLine)?;
    let towels = towels.split(", ").map(|s| s.as_bytes()).collect::<Vec<_>>();

    Ok(patterns
        .par_lines()
        .filter(|pattern| validate_pattern(pattern.as_bytes(), &towels))
        .count())
}

fn validate_pattern(pattern: &[u8], towels: &[&[u8]]) -> bool {
    pattern.is_empty()
        || towels.iter().any(|towel| {
            pattern.starts_with(towel) && validate_pattern(&pattern[towel.len()..], towels)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 6);
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
