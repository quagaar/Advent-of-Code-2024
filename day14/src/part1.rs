use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing position prefix")]
    MissingPositionPrefix,
    #[error("Missing velocity prefix")]
    MissingVelocityPrefix,
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Invalid input number: {0}")]
    InvalidInputNumber(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<i32, Error> {
    safety_factor(input, 101, 103)
}

fn safety_factor(input: &str, width: i32, height: i32) -> Result<i32, Error> {
    Ok(input
        .lines()
        .map(|line| Ok::<_, Error>(Robot::parse(line)?.simulate(width, height, 100)))
        .try_fold([0; 4], |mut acc, robot| {
            let quadrant = robot?.quadrant(width, height);
            if quadrant < 4 {
                acc[quadrant] += 1;
            }
            Ok::<_, Error>(acc)
        })?
        .into_iter()
        .product())
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn parse(line: &str) -> Result<Self, Error> {
        let (pos, vec) = line
            .strip_prefix("p=")
            .ok_or(Error::MissingPositionPrefix)?
            .split_once(" v=")
            .ok_or(Error::MissingVelocityPrefix)?;
        let (x, y) = pos.split_once(',').ok_or(Error::MissingDelimiter)?;
        let (dx, dy) = vec.split_once(',').ok_or(Error::MissingDelimiter)?;
        Ok(Robot {
            x: x.parse()?,
            y: y.parse()?,
            dx: dx.parse()?,
            dy: dy.parse()?,
        })
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
        let result = safety_factor(EXAMPLE, 11, 7).unwrap();
        assert_eq!(result, 12);
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
