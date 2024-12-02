use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    input.lines().filter(|x| is_safe(x)).count()
}

fn is_safe(line: &str) -> bool {
    line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
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
        let result = solve(EXAMPLE);
        assert_eq!(result, 2);
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
