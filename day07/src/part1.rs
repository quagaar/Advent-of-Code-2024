use rayon::prelude::*;

pub fn solve(input: &str) -> usize {
    input.par_lines().flat_map(process_line).sum()
}

fn process_line(line: &str) -> Option<usize> {
    let (test_value, numbers) = line.split_once(": ")?;
    let test_value = test_value.parse::<usize>().unwrap();
    let numbers = numbers
        .split_whitespace()
        .flat_map(|s| s.parse::<usize>())
        .collect::<Vec<_>>();

    if is_valid(test_value, numbers[0], &numbers[1..]) {
        Some(test_value)
    } else {
        None
    }
}

fn is_valid(target: usize, calc: usize, numbers: &[usize]) -> bool {
    if numbers.is_empty() {
        calc == target
    } else if calc > target {
        false
    } else {
        is_valid(target, calc * numbers[0], &numbers[1..])
            || is_valid(target, calc + numbers[0], &numbers[1..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 3749);
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
