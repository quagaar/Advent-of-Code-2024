pub fn solve(input: &str) -> u64 {
    let (left, right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .flat_map(|line| line.split_once("   "))
        .map(|(lhs, rhs)| (lhs.parse::<u64>().unwrap(), rhs.parse::<u64>().unwrap()))
        .unzip();
    left.into_iter()
        .map(|lhs| lhs * right.iter().filter(|rhs| lhs == **rhs).count() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 31);
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
