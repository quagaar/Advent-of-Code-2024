pub fn solve(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(parse_machine)
        .filter_map(calculate_cost)
        .sum()
}

fn parse_machine(input: &str) -> [i64; 6] {
    let mut lines = input.lines();
    let (ax, ay) = lines
        .next()
        .unwrap()
        .strip_prefix("Button A: X")
        .unwrap()
        .split_once(", Y")
        .unwrap();
    let (bx, by) = lines
        .next()
        .unwrap()
        .strip_prefix("Button B: X")
        .unwrap()
        .split_once(", Y")
        .unwrap();
    let (px, py) = lines
        .next()
        .unwrap()
        .strip_prefix("Prize: X=")
        .unwrap()
        .split_once(", Y=")
        .unwrap();
    [
        ax.parse().unwrap(),
        ay.parse().unwrap(),
        bx.parse().unwrap(),
        by.parse().unwrap(),
        px.parse().unwrap(),
        py.parse().unwrap(),
    ]
}

fn calculate_cost([ax, ay, bx, by, px, py]: [i64; 6]) -> Option<i64> {
    let px = px + 10000000000000;
    let py = py + 10000000000000;

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

    if b < 0 {
        // Button presses out of range
        return None;
    }

    let num = px - b * bx;

    if num % ax != 0 {
        // No whole number solution
        return None;
    }

    let a = num / ax;

    if a < 0 {
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
        let result = solve(EXAMPLE);
        assert_eq!(result, 875318608908);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
