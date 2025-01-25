use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse number: {0}")]
    FailedToParseNumber(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> Result<usize, Error> {
    let buttons = line.as_bytes();
    let code: usize = line.trim_start_matches('0').trim_end_matches('A').parse()?;
    let sequence = button_sequence(buttons);
    Ok(sequence.len() * code)
}

fn button_sequence(buttons: &[u8]) -> Vec<u8> {
    buttons
        .iter()
        .fold((b'A', vec![]), |(prev, mut sequence), &next| {
            let path = numeric_pad_paths(prev, next);
            sequence.extend_from_slice(path);
            (next, sequence)
        })
        .1
        .into_iter()
        .fold((b'A', vec![]), |(prev, mut sequence), next| {
            let path = direction_pad_path(prev, next);
            sequence.extend_from_slice(path);
            (next, sequence)
        })
        .1
        .into_iter()
        .fold((b'A', vec![]), |(prev, mut sequence), next| {
            let path = direction_pad_path(prev, next);
            sequence.extend_from_slice(path);
            (next, sequence)
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
    // use test_case::test_case;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 126384);
    }

    // #[test_case(
    //     "029A",
    //     "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
    // )]
    // #[test_case("980A", "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A")]
    // #[test_case(
    //     "179A",
    //     "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    // )]
    // #[test_case(
    //     "456A",
    //     "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"
    // )]
    // #[test_case(
    //     "379A",
    //     "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    // )]
    // fn example2(buttons: &str, expected: &str) {
    //     let result = button_sequence(buttons.as_bytes());
    //     let result = String::from_utf8(result).unwrap();
    //     assert_eq!(result, expected);
    // }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
