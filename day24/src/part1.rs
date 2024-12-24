use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let (start_values, gates) = input.split_once("\n\n").unwrap();
    let mut gates = gates.lines().map(parse_logic_gate).collect::<Vec<_>>();
    let mut values = HashMap::new();

    for line in start_values.lines() {
        let (name, value) = line.split_once(": ").unwrap();
        let value = match value {
            "1" => true,
            "0" => false,
            _ => panic!("Non boolean value!"),
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
    values
        .into_iter()
        .rev()
        .fold(0, |acc, (_, value)| acc * 2 + value as usize)
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

fn parse_logic_gate(line: &str) -> LogicGate<'_> {
    let (gate, output) = line.split_once(" -> ").unwrap();
    let mut parts = gate.split_whitespace();
    let lhs = parts.next().unwrap();
    let operation = match parts.next().unwrap() {
        "AND" => LogicOperation::And,
        "OR" => LogicOperation::Or,
        "XOR" => LogicOperation::Xor,
        _ => unreachable!(),
    };
    let rhs = parts.next().unwrap();
    LogicGate {
        lhs,
        rhs,
        output,
        operation,
    }
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
        let result = solve(EXAMPLE);
        assert_eq!(result, 4);
    }

    #[test]
    fn example2() {
        let result = solve(EXAMPLE2);
        assert_eq!(result, 2024);
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
