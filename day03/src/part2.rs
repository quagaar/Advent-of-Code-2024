use regex::Regex;
use std::sync::OnceLock;

static RE: OnceLock<Regex> = OnceLock::new();

pub fn solve(input: &str) -> usize {
    let rex =
        RE.get_or_init(|| Regex::new(r"(do\(\))|(don't\(\))|mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    let mut enabled = true;
    let mut result = 0;
    for c in rex.captures_iter(input) {
        if c.get(1).is_some() {
            enabled = true;
        } else if c.get(2).is_some() {
            enabled = false;
        } else if enabled {
            let lhs = c[3].parse::<usize>().unwrap();
            let rhs = c[4].parse::<usize>().unwrap();
            result += lhs * rhs;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 48);
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
