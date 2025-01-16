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
    let levels: Vec<_> = line
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .try_collect()?;

    Ok(safe_levels(levels.iter().copied())
        || (0..levels.len()).any(|n| {
            let mut levels = levels.clone();
            levels.remove(n);
            safe_levels(levels.into_iter())
        }))
}

fn safe_levels(levels: impl Iterator<Item = i32>) -> bool {
    levels
        .tuple_windows()
        .map(|(a, b)| a - b)
        .try_fold(0, |prev, diff| match diff {
            1..=3 => {
                if prev >= 0 {
                    Some(diff)
                } else {
                    None
                }
            }
            -3..=-1 => {
                if prev <= 0 {
                    Some(diff)
                } else {
                    None
                }
            }
            _ => None,
        })
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 4);
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
