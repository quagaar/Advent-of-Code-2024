use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No secret numbers found")]
    NoSecretNumbers,
    #[error("Failed to parse number: {0}")]
    FailedToParseNumber(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    input
        .par_lines()
        .map(get_sequences)
        .try_reduce_with(merge_maps)
        .ok_or(Error::NoSecretNumbers)??
        .into_values()
        .max()
        .ok_or(Error::NoSecretNumbers)
}

fn get_sequences(line: &str) -> Result<HashMap<[i8; 4], usize>, Error> {
    let mut secret_number: usize = line.parse()?;
    let mut last_digit = (secret_number % 10) as i8;
    Ok((0..2000)
        .map(move |_| {
            secret_number ^= secret_number << 6;
            secret_number &= (1 << 24) - 1;
            secret_number ^= secret_number >> 5;
            //secret_number &= (1 << 24) - 1;
            secret_number ^= secret_number << 11;
            secret_number &= (1 << 24) - 1;
            let prev = last_digit;
            let next = (secret_number % 10) as i8;
            last_digit = next;
            (prev - next, next)
        })
        .tuple_windows()
        .fold(HashMap::with_capacity(2000), |mut acc, (a, b, c, d)| {
            acc.entry([a.0, b.0, c.0, d.0]).or_insert(d.1 as usize);
            acc
        }))
}

fn merge_maps(
    mut a: HashMap<[i8; 4], usize>,
    mut b: HashMap<[i8; 4], usize>,
) -> Result<HashMap<[i8; 4], usize>, Error> {
    for (k, v) in b.drain() {
        *a.entry(k).or_default() += v;
    }
    Ok(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 23);
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
