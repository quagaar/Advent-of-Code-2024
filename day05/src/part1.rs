pub fn solve(input: &str) -> usize {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let rules = decode_rules(rules);

    pages
        .lines()
        .map(parse_pages)
        .filter(|pages| is_valid(pages, &rules))
        .map(|pages| middle_page(&pages))
        .sum()
}

fn decode_rules(rules: &str) -> [u128; 100] {
    let mut result = [0; 100];
    for line in rules.lines() {
        let (before, after) = line.split_once("|").unwrap();
        let before: usize = before.parse().unwrap();
        let after: usize = after.parse().unwrap();
        result[before] |= 1 << after;
    }
    result
}

fn parse_pages(line: &str) -> Vec<u8> {
    line.split(",").map(|n| n.parse().unwrap()).collect()
}

fn is_valid(pages: &[u8], rules: &[u128; 100]) -> bool {
    pages.iter().enumerate().skip(1).all(|(n, page)| {
        let after = rules[*page as usize];
        pages[..n].iter().all(|other| 0 == after & 1 << other)
    })
}

fn middle_page(pages: &[u8]) -> usize {
    pages[pages.len() / 2] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 143);
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
