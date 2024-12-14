pub fn solve(input: &str) -> i32 {
    safety_factor(input, 101, 103)
}

fn safety_factor(input: &str, width: i32, height: i32) -> i32 {
    input
        .lines()
        .map(|line| Robot::parse(line).simulate(width, height, 100))
        .fold([0; 4], |mut acc, robot| {
            let quadrant = robot.quadrant(width, height);
            if quadrant < 4 {
                acc[quadrant] += 1;
            }
            acc
        })
        .into_iter()
        .product()
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn parse(line: &str) -> Self {
        let (pos, vec) = line.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
        let (x, y) = pos.split_once(',').unwrap();
        let (dx, dy) = vec.split_once(',').unwrap();
        Robot {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            dx: dx.parse().unwrap(),
            dy: dy.parse().unwrap(),
        }
    }

    fn simulate(self, width: i32, height: i32, seconds: i32) -> Self {
        let dx = if self.dx < 0 {
            width + self.dx
        } else {
            self.dx
        };
        let dy = if self.dy < 0 {
            height + self.dy
        } else {
            self.dy
        };
        let x = (self.x + (dx * seconds)) % width;
        let y = (self.y + (dy * seconds)) % height;
        Robot {
            x,
            y,
            dx: self.dx,
            dy: self.dy,
        }
    }

    fn quadrant(&self, width: i32, height: i32) -> usize {
        if self.x < width / 2 && self.y < height / 2 {
            0
        } else if self.x > width / 2 && self.y < height / 2 {
            1
        } else if self.x < width / 2 && self.y > height / 2 {
            2
        } else if self.x > width / 2 && self.y > height / 2 {
            3
        } else {
            4
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = safety_factor(EXAMPLE, 11, 7);
        assert_eq!(result, 12);
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
