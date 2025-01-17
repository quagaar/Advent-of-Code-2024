use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub fn solve(input: &str) -> Result<usize, Error> {
    Ok(process_disk_map(input.trim_end().as_bytes())
        .enumerate()
        .map(|(position, id)| position * id)
        .sum())
}

fn process_disk_map(map: &[u8]) -> impl Iterator<Item = usize> + '_ {
    let mut low = 0;
    let mut low_count = (map[low] - b'0') as usize;
    let mut high = map.len() - 1;
    let mut high_count = (map[high] - b'0') as usize;

    std::iter::from_fn(move || {
        while low_count == 0 {
            low += 1;
            low_count = (map[low] - b'0') as usize;
        }
        while high_count == 0 && low < high {
            high -= 2;
            if low < high {
                high_count = (map[high] - b'0') as usize;
            } else {
                break;
            }
        }
        if low < high {
            if low & 1 == 0 {
                low_count -= 1;
                Some(low / 2)
            } else {
                high_count -= 1;
                low_count -= 1;
                Some(high / 2)
            }
        } else if high_count > 0 {
            high_count -= 1;
            Some(high / 2)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 1928);
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
