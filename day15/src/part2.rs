use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Robot not found")]
    RobotNotFound,
    #[error("Invalid move: {0}")]
    InvalidMove(char),
    #[error("Invalid tile: {0}")]
    InvalidTile(char),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let (map, moves) = input.split_once("\n\n").ok_or(Error::MissingDelimiter)?;
    let mut map: Vec<Vec<u8>> = map
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .flat_map(|tile| match tile {
                    b'#' => [Ok(b'#'), Ok(b'#')],
                    b'.' => [Ok(b'.'), Ok(b'.')],
                    b'O' => [Ok(b'['), Ok(b']')],
                    b'@' => [Ok(b'@'), Ok(b'.')],
                    _ => [Err(Error::InvalidTile(*tile as char)), Ok(*tile)],
                })
                .collect::<Result<_, Error>>()
        })
        .collect::<Result<_, Error>>()?;
    let moves = moves
        .lines()
        .flat_map(|line| line.as_bytes().iter().copied());
    let mut robot = find_robot(&map).ok_or(Error::RobotNotFound)?;

    for next in moves {
        move_robot(&mut map, &mut robot, next)?;
    }

    #[cfg(debug_assertions)]
    print_map(&map);

    Ok(box_gps_total(&map))
}

fn find_robot(map: &[Vec<u8>]) -> Option<(usize, usize)> {
    map.iter().enumerate().find_map(|(y, row)| {
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
}

fn move_robot(map: &mut Vec<Vec<u8>>, robot: &mut (usize, usize), next: u8) -> Result<(), Error> {
    let (dx, dy) = match next {
        b'^' => (0, -1),
        b'>' => (1, 0),
        b'v' => (0, 1),
        b'<' => (-1, 0),
        _ => return Err(Error::InvalidMove(next as char)),
    };

    let Some(x) = robot.0.checked_add_signed(dx) else {
        return Ok(());
    };
    let Some(y) = robot.1.checked_add_signed(dy) else {
        return Ok(());
    };

    if move_object(map, robot, dx, dy) {
        *robot = (x, y);
    }

    Ok(())
}

fn move_object(map: &mut Vec<Vec<u8>>, object: &(usize, usize), dx: isize, dy: isize) -> bool {
    if !can_move(map, object, dx, dy) {
        return false;
    }
    let Some(next) = get_next(object, dx, dy) else {
        return false;
    };
    match get_tile(map, object) {
        Some(b'@') => {
            move_object(map, &next, dx, dy);
            move_tile(map, object, &next);
        }
        Some(b'[') => {
            let other = (object.0 + 1, object.1);
            let other_next = (next.0 + 1, next.1);
            if dx >= 0 {
                move_object(map, &other_next, dx, dy);
                move_object(map, &next, dx, dy);
                move_tile(map, &other, &other_next);
                move_tile(map, object, &next);
            }
        }
        Some(b']') => {
            let other = (object.0 - 1, object.1);
            let other_next = (next.0 - 1, next.1);
            if dx <= 0 {
                move_object(map, &other_next, dx, dy);
                move_object(map, &next, dx, dy);
                move_tile(map, &other, &other_next);
                move_tile(map, object, &next);
            }
        }
        _ => (),
    }
    true
}

fn can_move(map: &[Vec<u8>], object: &(usize, usize), dx: isize, dy: isize) -> bool {
    let Some(next) = get_next(object, dx, dy) else {
        return false;
    };
    match get_tile(map, object) {
        Some(b'#') => false,
        Some(b'.') => true,
        Some(b'@') => can_move(map, &next, dx, dy),
        Some(b'[') => {
            if dx < 0 {
                can_move(map, &next, dx, dy)
            } else {
                let other = (next.0 + 1, next.1);
                if dx > 0 {
                    can_move(map, &other, dx, dy)
                } else {
                    can_move(map, &next, dx, dy) && can_move(map, &other, dx, dy)
                }
            }
        }
        Some(b']') => {
            if dx > 0 {
                can_move(map, &next, dx, dy)
            } else {
                let other = (next.0 - 1, next.1);
                if dx < 0 {
                    can_move(map, &other, dx, dy)
                } else {
                    can_move(map, &next, dx, dy) && can_move(map, &other, dx, dy)
                }
            }
        }
        _ => false,
    }
}

fn get_next(position: &(usize, usize), dx: isize, dy: isize) -> Option<(usize, usize)> {
    Some((
        position.0.checked_add_signed(dx)?,
        position.1.checked_add_signed(dy)?,
    ))
}

fn get_tile(map: &[Vec<u8>], object: &(usize, usize)) -> Option<u8> {
    map.get(object.1).and_then(|row| row.get(object.0)).copied()
}

fn move_tile(map: &mut [Vec<u8>], old: &(usize, usize), new: &(usize, usize)) {
    map[new.1][new.0] = map[old.1][old.0];
    map[old.1][old.0] = b'.';
}

fn box_gps_total(map: &[Vec<u8>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell == b'[')
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

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 9021);
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
