use std::collections::HashMap;

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

fn decode_rules(rules: &str) -> HashMap<usize, Vec<usize>> {
    rules.lines().fold(HashMap::new(), |mut acc, line| {
        let (before, after) = line.split_once("|").unwrap();
        let before: usize = before.parse().unwrap();
        let after: usize = after.parse().unwrap();
        acc.entry(before).or_default().push(after);
        acc
    })
}

fn parse_pages(line: &str) -> Vec<usize> {
    line.split(",").map(|n| n.parse().unwrap()).collect()
}

fn is_valid(pages: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    for n in 1..pages.len() {
        if let Some(after) = rules.get(&pages[n]) {
            if pages[..n].iter().any(|page| after.contains(page)) {
                return false;
            }
        }
    }
    true
}

fn middle_page(pages: &[usize]) -> usize {
    pages[pages.len() / 2]
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
