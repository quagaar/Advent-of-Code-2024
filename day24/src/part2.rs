pub fn solve(input: &str) -> String {
    let (_start_values, gates) = input.split_once("\n\n").unwrap();
    let gates = gates.lines().map(parse_logic_gate).collect::<Vec<_>>();

    let mut half_adds = vec![];
    let mut full_adds = vec![];
    let mut half_carries = vec![];
    let mut forward_carries = vec![];
    let mut full_carries = vec![];
    let mut z_max = "";

    for gate in gates {
        if gate.output.starts_with("z") {
            z_max = std::cmp::max(z_max, gate.output);
        }
        match gate.operation {
            LogicOperation::And => {
                if gate.lhs.starts_with("x") && gate.rhs.starts_with("y") {
                    half_carries.push((gate.lhs, gate.rhs, gate.output));
                } else if gate.lhs.starts_with("y") && gate.rhs.starts_with("x") {
                    half_carries.push((gate.rhs, gate.lhs, gate.output));
                } else {
                    forward_carries.push((gate.lhs, gate.rhs, gate.output));
                }
            }
            LogicOperation::Or => {
                full_carries.push((gate.lhs, gate.rhs, gate.output));
            }
            LogicOperation::Xor => {
                if gate.lhs.starts_with("x") && gate.rhs.starts_with("y") {
                    half_adds.push((gate.lhs, gate.rhs, gate.output));
                } else if gate.lhs.starts_with("y") && gate.rhs.starts_with("x") {
                    half_adds.push((gate.rhs, gate.lhs, gate.output));
                } else {
                    full_adds.push((gate.lhs, gate.rhs, gate.output));
                }
            }
        }
    }

    let mut results = vec![];

    for &(x, _y, half_carry) in half_carries.iter() {
        if x == "x00" {
            if !full_adds
                .iter()
                .any(|&(lhs, rhs, _)| lhs == half_carry || rhs == half_carry)
            {
                #[cfg(debug_assertions)]
                println!("half carry error: {}", half_carry);
                results.push(half_carry);
            }
        } else if !full_carries
            .iter()
            .any(|&(lhs, rhs, _)| lhs == half_carry || rhs == half_carry)
        {
            #[cfg(debug_assertions)]
            println!("half carry error: {}", half_carry);
            results.push(half_carry);
        }
    }

    for &(x, _y, half_add) in half_adds.iter() {
        if x == "x00" {
            if half_add != "z00" {
                #[cfg(debug_assertions)]
                println!("half add error: {}", half_add);
                results.push(half_add);
            }
        } else if !full_adds
            .iter()
            .any(|&(lhs, rhs, _)| lhs == half_add || rhs == half_add)
        {
            #[cfg(debug_assertions)]
            println!("half add error: {}", half_add);
            results.push(half_add);
        }
    }

    for &(_, _, forward_carry) in forward_carries.iter() {
        if !full_carries
            .iter()
            .any(|&(lhs, rhs, _)| lhs == forward_carry || rhs == forward_carry)
        {
            #[cfg(debug_assertions)]
            println!("forward carry error: {}", forward_carry);
            results.push(forward_carry);
        }
    }

    for &(_, _, full_carry) in full_carries.iter() {
        if full_carry != z_max
            && !full_adds
                .iter()
                .any(|&(lhs, rhs, _)| lhs == full_carry || rhs == full_carry)
        {
            #[cfg(debug_assertions)]
            println!("full carry error: {}", full_carry);
            results.push(full_carry);
        }
    }

    for &(_, _, full_add) in full_adds.iter() {
        if !full_add.starts_with("z") {
            #[cfg(debug_assertions)]
            println!("full add error: {}", full_add);
            results.push(full_add);
        }
    }

    results.sort();
    results.join(",")
}

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, "z00,z01");
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
