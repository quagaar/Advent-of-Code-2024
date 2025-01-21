use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing input line")]
    MissingInputLine,
    #[error("Missing button prefix")]
    MissingButtonPrefix,
    #[error("Missing prize prefix")]
    MissingPrizePrefix,
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Invalid input number: {0}")]
    InvalidInputNumber(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<i32, Error> {
    input
        .split("\n\n")
        .map(parse_machine)
        .try_fold(0, |acc, machine| {
            if let Some(cost) = calculate_cost(machine?) {
                Ok(acc + cost)
            } else {
                Ok(acc)
            }
        })
}

fn parse_machine(input: &str) -> Result<[i32; 6], Error> {
    let mut lines = input.lines();
    let (ax, ay) = lines
        .next()
        .ok_or(Error::MissingInputLine)?
        .strip_prefix("Button A: X")
        .ok_or(Error::MissingButtonPrefix)?
        .split_once(", Y")
        .ok_or(Error::MissingDelimiter)?;
    let (bx, by) = lines
        .next()
        .ok_or(Error::MissingInputLine)?
        .strip_prefix("Button B: X")
        .ok_or(Error::MissingButtonPrefix)?
        .split_once(", Y")
        .ok_or(Error::MissingDelimiter)?;
    let (px, py) = lines
        .next()
        .ok_or(Error::MissingInputLine)?
        .strip_prefix("Prize: X=")
        .ok_or(Error::MissingPrizePrefix)?
        .split_once(", Y=")
        .ok_or(Error::MissingDelimiter)?;
    Ok([
        ax.parse()?,
        ay.parse()?,
        bx.parse()?,
        by.parse()?,
        px.parse()?,
        py.parse()?,
    ])
}

fn calculate_cost([ax, ay, bx, by, px, py]: [i32; 6]) -> Option<i32> {
    let num = ax * py - ay * px;
    let den = ax * by - ay * bx;

    if num == 0 && den == 0 {
        // Vector are all parallel
        panic!("Degenerate case");
    }

    if den == 0 {
        // Button vectors have same angle, but different from prize vector
        // No solution possible
        return None;
    }

    if num % den != 0 {
        // No whole number solution
        return None;
    }

    let b = num / den;

    if !(0..=100).contains(&b) {
        // Button presses out of range
        return None;
    }

    let num = px - b * bx;

    if num % ax != 0 {
        // No whole number solution
        return None;
    }

    let a = num / ax;

    if !(0..=100).contains(&a) {
        // Button presses out of range
        return None;
    }

    Some(a * 3 + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 480);
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
