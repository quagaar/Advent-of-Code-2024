pub fn solve(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(parse_machine)
        .filter_map(calculate_cost)
        .sum()
}

fn parse_machine(input: &str) -> [i32; 6] {
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
        let result = solve(EXAMPLE);
        assert_eq!(result, 480);
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
