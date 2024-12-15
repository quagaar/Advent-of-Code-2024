pub fn solve(input: &str) -> usize {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<u8>> = map.lines().map(|line| line.as_bytes().to_vec()).collect();
    let moves = moves
        .lines()
        .flat_map(|line| line.as_bytes().iter().copied());
    let mut robot = find_robot(&map);

    for next in moves {
        move_robot(&mut map, &mut robot, next);
    }

    #[cfg(debug_assertions)]
    print_map(&map);

    box_gps_total(&map)
}

fn find_robot(map: &[Vec<u8>]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(
                move |(x, &cell)| {
                    if cell == b'@' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .unwrap()
}

fn move_robot(map: &mut Vec<Vec<u8>>, robot: &mut (usize, usize), next: u8) {
    let (dx, dy) = match next {
        b'^' => (0, -1),
        b'>' => (1, 0),
        b'v' => (0, 1),
        b'<' => (-1, 0),
        _ => panic!("Invalid move"),
    };

    if move_object(map, robot, dx, dy) {
        *robot = (
            robot.0.checked_add_signed(dx).unwrap(),
            robot.1.checked_add_signed(dy).unwrap(),
        );
    }
}

fn move_object(map: &mut Vec<Vec<u8>>, object: &(usize, usize), dx: isize, dy: isize) -> bool {
    let next = (
        object.0.checked_add_signed(dx).unwrap(),
        object.1.checked_add_signed(dy).unwrap(),
    );
    match map[next.1][next.0] {
        b'#' => false,
        b'.' => {
            map[next.1][next.0] = map[object.1][object.0];
            map[object.1][object.0] = b'.';
            true
        }
        b'O' => {
            if move_object(map, &next, dx, dy) {
                map[next.1][next.0] = map[object.1][object.0];
                map[object.1][object.0] = b'.';
                true
            } else {
                false
            }
        }
        _ => panic!("Invalid cell"),
    }
}

fn box_gps_total(map: &[Vec<u8>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell == b'O')
                .map(move |(x, _)| (x, y))
        })
        .map(|(x, y)| 100 * y + x)
        .sum()
}

#[allow(dead_code)]
fn print_map(map: &[Vec<u8>]) {
    for row in map {
        for cell in row {
            print!("{}", *cell as char);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 10092);
    }

    #[test]
    fn example2() {
        let result = solve(EXAMPLE2);
        assert_eq!(result, 2028);
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
