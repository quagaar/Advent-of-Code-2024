use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    input.lines().filter(|x| is_safe(x)).count()
}

fn is_safe(line: &str) -> bool {
    let levels = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();

    safe_levels(levels.iter().copied())
        || (0..levels.len()).any(|n| {
            let mut levels = levels.clone();
            levels.remove(n);
            safe_levels(levels.into_iter())
        })
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
        let result = solve(EXAMPLE);
        assert_eq!(result, 4);
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
