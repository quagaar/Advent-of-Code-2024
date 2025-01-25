use rayon::prelude::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse number: {0}")]
    FailedToParseNumber(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    input.par_lines().map(process_line).sum()
}

fn process_line(line: &str) -> Result<usize, Error> {
    let buttons = line.as_bytes();
    let code: usize = line.trim_start_matches('0').trim_end_matches('A').parse()?;
    let sequence_length = button_sequence_length(buttons);
    Ok(sequence_length * code)
}

fn button_sequence_length(buttons: &[u8]) -> usize {
    let number_pad_sequence = buttons
        .iter()
        .fold((b'A', vec![]), |(prev, mut sequence), &next| {
            let path = numeric_pad_paths(prev, next);
            sequence.extend_from_slice(path);
            (next, sequence)
        })
        .1;

    let mut memo = HashMap::new();

    expand_sequence(&number_pad_sequence, 0, &mut memo)
}

fn expand_sequence(sequence: &[u8], robots: u8, memo: &mut HashMap<(u8, u8, u8), usize>) -> usize {
    if robots == 25 {
        return sequence.len();
    }
    sequence
        .iter()
        .fold((b'A', 0), |(prev, total), &button| {
            if let Some(&len) = memo.get(&(prev, button, robots)) {
                (button, total + len)
            } else {
                let len = expand_sequence(direction_pad_path(prev, button), robots + 1, memo);
                memo.insert((prev, button, robots), len);
                (button, total + len)
            }
        })
        .1
}

fn numeric_pad_paths(start: u8, end: u8) -> &'static [u8] {
    match start {
        b'A' => match end {
            b'A' => "A".as_bytes(),
            b'0' => "<A".as_bytes(),
            b'1' => "^<<A".as_bytes(),
            b'2' => "<^A".as_bytes(),
            b'3' => "^A".as_bytes(),
            b'4' => "^^<<A".as_bytes(),
            b'5' => "<^^A".as_bytes(),
            b'6' => "^^A".as_bytes(),
            b'7' => "^^^<<A".as_bytes(),
            b'8' => "<^^^A".as_bytes(),
            b'9' => "^^^A".as_bytes(),
            _ => &[],
        },
        b'0' => match end {
            b'A' => ">A".as_bytes(),
            b'0' => "<A".as_bytes(),
            b'1' => "^<A".as_bytes(),
            b'2' => "^A".as_bytes(),
            b'3' => "^>A".as_bytes(),
            b'4' => "^^<A".as_bytes(),
            b'5' => "^^A".as_bytes(),
            b'6' => "^^>A".as_bytes(),
            b'7' => "^^^<A".as_bytes(),
            b'8' => "^^^A".as_bytes(),
            b'9' => "^^^>A".as_bytes(),
            _ => &[],
        },
        b'1' => match end {
            b'A' => ">>vA".as_bytes(),
            b'0' => ">vA".as_bytes(),
            b'1' => "A".as_bytes(),
            b'2' => ">A".as_bytes(),
            b'3' => ">>A".as_bytes(),
            b'4' => "^A".as_bytes(),
            b'5' => "^>A".as_bytes(),
            b'6' => "^>>A".as_bytes(),
            b'7' => "^^A".as_bytes(),
            b'8' => "^^>A".as_bytes(),
            b'9' => "^^>>A".as_bytes(),
            _ => &[],
        },
        b'2' => match end {
            b'A' => "v>A".as_bytes(),
            b'0' => "vA".as_bytes(),
            b'1' => "<A".as_bytes(),
            b'2' => "A".as_bytes(),
            b'3' => ">A".as_bytes(),
            b'4' => "<^A".as_bytes(),
            b'5' => "^A".as_bytes(),
            b'6' => "^>A".as_bytes(),
            b'7' => "<^^A".as_bytes(),
            b'8' => "^^A".as_bytes(),
            b'9' => "^^>A".as_bytes(),
            _ => &[],
        },
        b'3' => match end {
            b'A' => "vA".as_bytes(),
            b'0' => "<vA".as_bytes(),
            b'1' => "<<A".as_bytes(),
            b'2' => "<A".as_bytes(),
            b'3' => "A".as_bytes(),
            b'4' => "<<^A".as_bytes(),
            b'5' => "<^A".as_bytes(),
            b'6' => "^A".as_bytes(),
            b'7' => "<<^^A".as_bytes(),
            b'8' => "<^^A".as_bytes(),
            b'9' => "^^A".as_bytes(),
            _ => &[],
        },
        b'4' => match end {
            b'A' => ">>vvA".as_bytes(),
            b'0' => ">vvA".as_bytes(),
            b'1' => "vA".as_bytes(),
            b'2' => "v>A".as_bytes(),
            b'3' => "v>>A".as_bytes(),
            b'4' => "A".as_bytes(),
            b'5' => ">A".as_bytes(),
            b'6' => ">>A".as_bytes(),
            b'7' => "^A".as_bytes(),
            b'8' => "^>A".as_bytes(),
            b'9' => "^>>A".as_bytes(),
            _ => &[],
        },
        b'5' => match end {
            b'A' => "vv>A".as_bytes(),
            b'0' => "vvA".as_bytes(),
            b'1' => "<vA".as_bytes(),
            b'2' => "vA".as_bytes(),
            b'3' => "v>A".as_bytes(),
            b'4' => "<A".as_bytes(),
            b'5' => "A".as_bytes(),
            b'6' => ">A".as_bytes(),
            b'7' => "<^A".as_bytes(),
            b'8' => "^A".as_bytes(),
            b'9' => "^>A".as_bytes(),
            _ => &[],
        },
        b'6' => match end {
            b'A' => "vvA".as_bytes(),
            b'0' => "<vvA".as_bytes(),
            b'1' => "<<vA".as_bytes(),
            b'2' => "<vA".as_bytes(),
            b'3' => "vA".as_bytes(),
            b'4' => "<<A".as_bytes(),
            b'5' => "<A".as_bytes(),
            b'6' => "A".as_bytes(),
            b'7' => "<<^A".as_bytes(),
            b'8' => "<^A".as_bytes(),
            b'9' => "^A".as_bytes(),
            _ => &[],
        },
        b'7' => match end {
            b'A' => ">>vvvA".as_bytes(),
            b'0' => ">vvvA".as_bytes(),
            b'1' => "vvA".as_bytes(),
            b'2' => "vv>A".as_bytes(),
            b'3' => "vv>>A".as_bytes(),
            b'4' => "vA".as_bytes(),
            b'5' => "v>A".as_bytes(),
            b'6' => "v>>A".as_bytes(),
            b'7' => "A".as_bytes(),
            b'8' => ">A".as_bytes(),
            b'9' => ">>A".as_bytes(),
            _ => &[],
        },
        b'8' => match end {
            b'A' => "vvv>A".as_bytes(),
            b'0' => "vvvA".as_bytes(),
            b'1' => "<vvA".as_bytes(),
            b'2' => "vvA".as_bytes(),
            b'3' => "vv>A".as_bytes(),
            b'4' => "<vA".as_bytes(),
            b'5' => "vA".as_bytes(),
            b'6' => "v>A".as_bytes(),
            b'7' => "<A".as_bytes(),
            b'8' => "A".as_bytes(),
            b'9' => ">A".as_bytes(),
            _ => &[],
        },
        b'9' => match end {
            b'A' => "vvvA".as_bytes(),
            b'0' => "<vvvA".as_bytes(),
            b'1' => "<<vvA".as_bytes(),
            b'2' => "<vvA".as_bytes(),
            b'3' => "vvA".as_bytes(),
            b'4' => "<<vA".as_bytes(),
            b'5' => "<vA".as_bytes(),
            b'6' => "vA".as_bytes(),
            b'7' => "<<A".as_bytes(),
            b'8' => "<A".as_bytes(),
            b'9' => "A".as_bytes(),
            _ => &[],
        },
        _ => &[],
    }
}

fn direction_pad_path(start: u8, end: u8) -> &'static [u8] {
    match start {
        b'^' => match end {
            b'^' => "A".as_bytes(),
            b'A' => ">A".as_bytes(),
            b'<' => "v<A".as_bytes(),
            b'v' => "vA".as_bytes(),
            b'>' => "v>A".as_bytes(),
            _ => &[],
        },
        b'A' => match end {
            b'^' => "<A".as_bytes(),
            b'A' => "A".as_bytes(),
            b'<' => "v<<A".as_bytes(),
            b'v' => "<vA".as_bytes(),
            b'>' => "vA".as_bytes(),
            _ => &[],
        },
        b'<' => match end {
            b'^' => ">^A".as_bytes(),
            b'A' => ">>^A".as_bytes(),
            b'<' => "A".as_bytes(),
            b'v' => ">A".as_bytes(),
            b'>' => ">>A".as_bytes(),
            _ => &[],
        },
        b'v' => match end {
            b'^' => "^A".as_bytes(),
            b'A' => "^>A".as_bytes(),
            b'<' => "<A".as_bytes(),
            b'v' => "A".as_bytes(),
            b'>' => ">A".as_bytes(),
            _ => &[],
        },
        b'>' => match end {
            b'^' => "<^A".as_bytes(),
            b'A' => "^A".as_bytes(),
            b'<' => "<<A".as_bytes(),
            b'v' => "<A".as_bytes(),
            b'>' => "A".as_bytes(),
            _ => &[],
        },
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 154115708116294);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
