use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    input
        .par_lines()
        .map(get_sequences)
        .reduce_with(|mut a, mut b| {
            for (k, v) in b.drain() {
                *a.entry(k).or_default() += v;
            }
            a
        })
        .unwrap()
        .into_values()
        .max()
        .unwrap()
}

fn get_sequences(line: &str) -> HashMap<[i8; 4], usize> {
    let mut secret_number: usize = line.parse().unwrap();
    let mut last_digit = (secret_number % 10) as i8;
    (0..2000)
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
        .fold(HashMap::new(), |mut acc, (a, b, c, d)| {
            acc.entry([a.0, b.0, c.0, d.0]).or_insert(d.1 as usize);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 23);
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
