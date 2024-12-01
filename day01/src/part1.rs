use itertools::Itertools;

pub fn solve(input: &str) -> u64 {
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = input
        .split_whitespace()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap()))
        .map(|(lhs, rhs)| (lhs.parse::<u64>().unwrap(), rhs.parse::<u64>().unwrap()))
        .unzip();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(lhs, rhs)| lhs.abs_diff(rhs))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 11);
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
