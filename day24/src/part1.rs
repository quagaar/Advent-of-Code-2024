use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing blank line")]
    MissingBlankLine,
    #[error("Missing input value delimiter")]
    MissingInputValueDelimiter,
    #[error("Missing gate output delimiter")]
    MissingGateOutputDelimiter,
    #[error("Missing gate left hand side")]
    MissingGateLeftHandSide,
    #[error("Missing gate operation")]
    MissingGateOperation,
    #[error("Invalid gate operation: {0}")]
    InvalidGateOperation(String),
    #[error("Missing gate right hand side")]
    MissingGateRightHandSide,
    #[error("Non boolean value: {0}")]
    NonBooleanValue(String),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let (start_values, gates) = input.split_once("\n\n").ok_or(Error::MissingBlankLine)?;
    let mut gates = gates
        .lines()
        .map(parse_logic_gate)
        .collect::<Result<Vec<_>, Error>>()?;
    let mut values = HashMap::new();

    for line in start_values.lines() {
        let (name, value) = line
            .split_once(": ")
            .ok_or(Error::MissingInputValueDelimiter)?;
        let value = match value {
            "1" => true,
            "0" => false,
            _ => return Err(Error::NonBooleanValue(value.to_owned())),
        };
        values.insert(name, value);
    }

    while !gates.is_empty() {
        gates = gates
            .into_iter()
            .filter_map(|gate| {
                if let Some((out, result)) = apply_logic_gate(&gate, &values) {
                    values.insert(out, result);
                    None
                } else {
                    Some(gate)
                }
            })
            .collect();
    }

    let mut values = values
        .into_iter()
        .filter(|(name, _)| name.starts_with("z"))
        .collect::<Vec<_>>();
    values.sort();
    Ok(values
        .into_iter()
        .rev()
        .fold(0, |acc, (_, value)| acc * 2 + value as usize))
}

enum LogicOperation {
    And,
    Or,
    Xor,
}

struct LogicGate<'a> {
    lhs: &'a str,
    rhs: &'a str,
    output: &'a str,
    operation: LogicOperation,
}

fn parse_logic_gate(line: &str) -> Result<LogicGate<'_>, Error> {
    let (gate, output) = line
        .split_once(" -> ")
        .ok_or(Error::MissingGateOutputDelimiter)?;
    let mut parts = gate.split_whitespace();
    let lhs = parts.next().ok_or(Error::MissingGateLeftHandSide)?;
    let operation = match parts.next().ok_or(Error::MissingGateOperation)? {
        "AND" => LogicOperation::And,
        "OR" => LogicOperation::Or,
        "XOR" => LogicOperation::Xor,
        op => return Err(Error::InvalidGateOperation(op.to_owned())),
    };
    let rhs = parts.next().ok_or(Error::MissingGateRightHandSide)?;
    Ok(LogicGate {
        lhs,
        rhs,
        output,
        operation,
    })
}

fn apply_logic_gate<'a>(
    gate: &LogicGate<'a>,
    values: &HashMap<&'a str, bool>,
) -> Option<(&'a str, bool)> {
    let lhs = *values.get(gate.lhs)?;
    let rhs = *values.get(gate.rhs)?;
    let result = match gate.operation {
        LogicOperation::And => lhs & rhs,
        LogicOperation::Or => lhs | rhs,
        LogicOperation::Xor => lhs ^ rhs,
    };
    Some((gate.output, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn example2() {
        let result = solve(EXAMPLE2).unwrap();
        assert_eq!(result, 2024);
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
