use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse number: {0}")]
    FailedToParseNumber(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    input.par_lines().map(process_line).sum()
}

fn process_line(line: &str) -> Result<usize, Error> {
    let mut secret_number = line.parse()?;
    for _ in 0..2000 {
        secret_number ^= secret_number << 6;
        secret_number &= (1 << 24) - 1;
        secret_number ^= secret_number >> 5;
        //secret_number &= (1 << 24) - 1;
        secret_number ^= secret_number << 11;
        //secret_number &= (1 << 24) - 1;
    }
    secret_number &= (1 << 24) - 1;
    Ok(secret_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn prerequisites() {
        assert_eq!(123 * 64, 123 << 6);
        assert_eq!(123456789 / 32, 123456789 >> 5);
        assert_eq!(123 * 2048, 123 << 11);
        assert_eq!(16777216, 1 << 24);
        assert_eq!(123456789 % 16777216, 123456789 & ((1 << 24) - 1));
    }

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 37327623);
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
