pub fn solve(input: &str) -> usize {
    let mut keys = vec![];
    let mut locks = vec![];

    for schematic in input.split("\n\n") {
        let mut lines = schematic.lines();
        let Some(first_line) = lines.next() else {
            continue;
        };

        if first_line == "#####" {
            let mut lock = [6; 5];
            for line in lines {
                for (col, lock_col) in lock.iter_mut().enumerate() {
                    if Some(&b'.') == line.as_bytes().get(col) {
                        *lock_col -= 1;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = [5; 5];
            for line in lines {
                for (col, key_col) in key.iter_mut().enumerate() {
                    if Some(&b'.') == line.as_bytes().get(col) {
                        *key_col -= 1;
                    }
                }
            }
            keys.push(key);
        }
    }

    keys.into_iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| (0..5).all(|index| key[index] + lock[index] <= 5))
                .count()
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
        assert_eq!(result, 3);
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
