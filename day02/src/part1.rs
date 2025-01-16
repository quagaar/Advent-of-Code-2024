use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    input
        .lines()
        .try_fold(0, |acc, x| if is_safe(x)? { Ok(acc + 1) } else { Ok(acc) })
}

fn is_safe(line: &str) -> Result<bool, Error> {
    let mut prev = 0;
    for diff in line
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .tuple_windows()
        .map(|(a, b)| Ok::<_, Error>(a? - b?))
    {
        let diff = diff?;
        match diff {
            1..=3 => {
                if prev >= 0 {
                    prev = diff;
                } else {
                    return Ok(false);
                }
            }
            -3..=-1 => {
                if prev <= 0 {
                    prev = diff;
                } else {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 2);
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
