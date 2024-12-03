use regex::Regex;
use std::sync::OnceLock;

static RE: OnceLock<Regex> = OnceLock::new();

pub fn solve(input: &str) -> usize {
    let rex = RE.get_or_init(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    rex.captures_iter(input)
        .map(|c| {
            let lhs = c[1].parse::<usize>().unwrap();
            let rhs = c[2].parse::<usize>().unwrap();
            lhs * rhs
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 161);
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
