use rayon::prelude::*;

pub fn solve(input: &str) -> usize {
    find_easter_egg(input, 101, 103)
}

fn find_easter_egg(input: &str, width: i32, height: i32) -> usize {
    let mut robots = input
        .lines()
        .map(|line| Robot::parse(line, width, height))
        .collect::<Vec<_>>();

    let limit = robots.len() * 70 / 100;

    for seconds in 0.. {
        let neighbours = count_neighbours(&robots);
        if neighbours > limit {
            #[cfg(debug_assertions)]
            print_grid(&robots, width, height);
            #[cfg(debug_assertions)]
            println!("neighbours: {} > limit: {}", neighbours, limit);
            return seconds;
        }
        robots
            .iter_mut()
            .for_each(|robot| *robot = robot.next(width, height));
    }

    unreachable!("no solution found")
}

fn count_neighbours(robots: &[Robot]) -> usize {
    robots
        .par_iter()
        .enumerate()
        .map(|(i, robot)| robot.count_neighbours(&robots[i + 1..]))
        .sum()
}

#[allow(dead_code)]
fn print_grid(robots: &[Robot], width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            let count = robots
                .iter()
                .filter(|robot| robot.x == x && robot.y == y)
                .count();
            if count > 0 {
                print!("{}", count);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn parse(line: &str, width: i32, height: i32) -> Self {
        let (pos, vec) = line.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
        let (x, y) = pos.split_once(',').unwrap();
        let (dx, dy) = vec.split_once(',').unwrap();
        let mut dx = dx.parse().unwrap();
        let mut dy = dy.parse().unwrap();
        if dx < 0 {
            dx += width
        }
        if dy < 0 {
            dy += height
        }
        Robot {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            dx,
            dy,
        }
    }

    fn next(self, width: i32, height: i32) -> Self {
        let x = (self.x + self.dx) % width;
        let y = (self.y + self.dy) % height;
        Robot {
            x,
            y,
            dx: self.dx,
            dy: self.dy,
        }
    }

    fn is_neighbour(&self, other: &Self) -> bool {
        matches!(
            (self.x.abs_diff(other.x), (self.y.abs_diff(other.y))),
            (1, 0) | (0, 1)
        )
    }

    fn count_neighbours(&self, robots: &[Self]) -> usize {
        robots
            .iter()
            .filter(|robot| self.is_neighbour(robot))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
