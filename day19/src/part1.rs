pub fn solve(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect::<Vec<_>>();

    patterns
        .lines()
        .filter(|pattern| validate_pattern(pattern, &towels))
        .count()
}

fn validate_pattern(pattern: &str, towels: &[&str]) -> bool {
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
        let result = solve(EXAMPLE);
        assert_eq!(result, 6);
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
