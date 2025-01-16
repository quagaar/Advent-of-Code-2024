use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to split input")]
    UnableToSplitInput,
    #[error("Missing rule delimiter")]
    MissingRuleDelimiter,
    #[error("Unable to parse number, reason: {0}")]
    UnableToParseNumber(#[from] ParseIntError),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let (rules, pages) = input.split_once("\n\n").ok_or(Error::UnableToSplitInput)?;
    let rules = decode_rules(rules)?;

    let mut total = 0;
    for line in pages.lines() {
        let pages = parse_pages(line)?;
        if !is_valid(&pages, &rules) {
            let pages = sort_pages(pages, &rules);
            total += middle_page(&pages);
        }
    }
    Ok(total)
}

fn decode_rules(rules: &str) -> Result<[u128; 100], Error> {
    let mut result = [0; 100];
    for line in rules.lines() {
        let (before, after) = line.split_once("|").ok_or(Error::MissingRuleDelimiter)?;
        let before: usize = before.parse()?;
        let after: usize = after.parse()?;
        result[before] |= 1 << after;
    }
    Ok(result)
}

fn parse_pages(line: &str) -> Result<Vec<u8>, ParseIntError> {
    line.split(",").map(|n| n.parse()).collect()
}

fn is_valid(pages: &[u8], rules: &[u128; 100]) -> bool {
    pages.iter().enumerate().skip(1).all(|(n, page)| {
        let after = rules[*page as usize];
        pages[..n].iter().all(|other| 0 == after & 1 << other)
    })
}

fn sort_pages(mut pages: Vec<u8>, rules: &[u128; 100]) -> Vec<u8> {
    let mut stack = vec![];
    let mut sorted = Vec::with_capacity(pages.len());
    while !stack.is_empty() || !pages.is_empty() {
        if stack.is_empty() {
            stack.push(pages.remove(0));
        }
        if let Some(&current_page) = stack.last() {
            let after = rules[current_page as usize];
            if let Some(next) = pages.iter().position(|other| 0 != after & 1 << other) {
                stack.push(pages.remove(next));
            } else {
                sorted.push(stack.pop().unwrap());
            }
        }
    }
    sorted
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
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 123);
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
