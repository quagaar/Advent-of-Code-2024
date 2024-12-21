use pathfinding::prelude::dijkstra;

pub fn solve(input: &str) -> usize {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> usize {
    let buttons = line.as_bytes();
    let code: usize = line
        .trim_start_matches('0')
        .trim_end_matches('A')
        .parse()
        .unwrap();
    let sequence = button_sequence(buttons);
    sequence.len() * code
}

fn button_sequence(buttons: &[u8]) -> Vec<u8> {
    buttons
        .iter()
        .try_fold(
            (b'A', vec![]),
            |(prev_button, mut sequence), &next_button| {
                let (path, _len) = dijkstra(
                    &(prev_button, b'A', b'A', b'\0'),
                    |&(a, b, c, _)| successors(a, b, c),
                    |&(a, b, c, d)| a == next_button && b == b'A' && c == b'A' && d == b'A',
                )?;
                for (_, _, _, button) in path.into_iter().skip(1) {
                    sequence.push(button);
                }
                Some((next_button, sequence))
            },
        )
        .expect("Unable to find button sequence")
        .1
}

fn successors(a: u8, b: u8, c: u8) -> Vec<((u8, u8, u8, u8), usize)> {
    let mut result = vec![];
    match (b, c) {
        (b'A', b'A') => {
            result.push(((a, b, c, b'A'), 1));
        }
        (_, b'A') => {
            if let Some(x) = numeric_keypad_move(a, b) {
                result.push(((x, b, c, b'A'), 1));
            }
        }
        (_, _) => {
            if let Some(x) = directional_keypad_move(b, c) {
                result.push(((a, x, c, b'A'), 1));
            }
        }
    }
    for &button in "<>v^".as_bytes() {
        if let Some(x) = directional_keypad_move(c, button) {
            result.push(((a, b, x, button), 1));
        }
    }
    result
}

// fn numeric_keypad_next(button: u8) -> &'static [(u8, u8)] {
//     match button {
//         b'A' => &[(b'0', b'<'), (b'3', b'^')],
//         b'1' => &[(b'2', b'>'), (b'4', b'^')],
//         b'2' => &[(b'1', b'<'), (b'3', b'>'), (b'5', b'^')],
//         b'3' => &[(b'2', b'<'), (b'6', b'^'), (b'A', b'v')],
//         b'4' => &[(b'1', b'v'), (b'5', b'>'), (b'7', b'^')],
//         b'5' => &[(b'2', b'v'), (b'4', b'<'), (b'6', b'>'), (b'8', b'^')],
//         b'6' => &[(b'3', b'v'), (b'5', b'<'), (b'9', b'^')],
//         b'7' => &[(b'4', b'v'), (b'8', b'>')],
//         b'8' => &[(b'5', b'v'), (b'7', b'<'), (b'9', b'>')],
//         b'9' => &[(b'6', b'v'), (b'8', b'<')],
//         b'0' => &[(b'2', b'^'), (b'A', b'>')],
//         _ => panic!("Invalid button: {}", button),
//     }
// }

fn numeric_keypad_move(button: u8, direction: u8) -> Option<u8> {
    match direction {
        b'^' => match button {
            b'4' => Some(b'7'),
            b'5' => Some(b'8'),
            b'6' => Some(b'9'),
            b'1' => Some(b'4'),
            b'2' => Some(b'5'),
            b'3' => Some(b'6'),
            b'0' => Some(b'2'),
            b'A' => Some(b'3'),
            _ => None,
        },
        b'<' => match button {
            b'8' => Some(b'7'),
            b'9' => Some(b'8'),
            b'5' => Some(b'4'),
            b'6' => Some(b'5'),
            b'2' => Some(b'1'),
            b'3' => Some(b'2'),
            b'A' => Some(b'0'),
            _ => None,
        },
        b'v' => match button {
            b'7' => Some(b'4'),
            b'8' => Some(b'5'),
            b'9' => Some(b'6'),
            b'4' => Some(b'1'),
            b'5' => Some(b'2'),
            b'6' => Some(b'3'),
            b'2' => Some(b'0'),
            b'3' => Some(b'A'),
            _ => None,
        },
        b'>' => match button {
            b'7' => Some(b'8'),
            b'8' => Some(b'9'),
            b'4' => Some(b'5'),
            b'5' => Some(b'6'),
            b'1' => Some(b'2'),
            b'2' => Some(b'3'),
            b'0' => Some(b'A'),
            _ => None,
        },
        _ => None,
    }
}

// fn directional_keypad_next(button: u8) -> &'static [u8] {
//     match button {
//         b'A' => &[b'^', b'>'],
//         b'^' => &[b'v', b'A'],
//         b'<' => &[b'v'],
//         b'v' => &[b'<', b'^', b'>'],
//         b'>' => &[b'v', b'A'],
//         _ => panic!("Invalid button: {}", button),
//     }
// }

fn directional_keypad_move(button: u8, direction: u8) -> Option<u8> {
    match direction {
        b'^' => match button {
            b'>' => Some(b'A'),
            b'v' => Some(b'^'),
            _ => None,
        },
        b'<' => match button {
            b'A' => Some(b'^'),
            b'>' => Some(b'v'),
            b'v' => Some(b'<'),
            _ => None,
        },
        b'v' => match button {
            b'A' => Some(b'>'),
            b'^' => Some(b'v'),
            _ => None,
        },
        b'>' => match button {
            b'^' => Some(b'A'),
            b'<' => Some(b'v'),
            b'v' => Some(b'>'),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test_case::test_case;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
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
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
