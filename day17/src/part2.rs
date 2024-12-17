pub fn solve(input: &str) -> usize {
    let computer = Computer::parse(input).expect("Unable to parse input");

    #[cfg(debug_assertions)]
    println!("{}", computer.disassemble());

    computer
        .program
        .iter()
        .rev()
        .fold(vec![0], |acc, next| {
            acc.into_iter()
                .flat_map(|a| (0..8).map(move |n| a << 3 | n))
                .filter(|&a| {
                    let mut computer = computer.clone();
                    computer.a = a;
                    computer
                        .run()
                        .map_or(false, |output| output.first() == Some(next))
                })
                .collect()
        })
        .first()
        .copied()
        .expect("Not found")
}

#[derive(Debug, Clone)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    program: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Computer {
    fn parse(input: &str) -> Option<Self> {
        let mut lines = input.lines();
        let a = lines.next()?.strip_prefix("Register A: ")?.parse().ok()?;
        let b = lines.next()?.strip_prefix("Register B: ")?.parse().ok()?;
        let c = lines.next()?.strip_prefix("Register C: ")?.parse().ok()?;
        lines.next()?;
        let program = lines
            .next()?
            .strip_prefix("Program: ")?
            .split(',')
            .flat_map(|s| s.parse())
            .collect();
        Some(Self {
            a,
            b,
            c,
            ip: 0,
            program,
        })
    }

    #[allow(dead_code)]
    fn disassemble(&self) -> String {
        self.program
            .chunks_exact(2)
            .map(|chunk| {
                let opcode = Opcode::from(chunk[0]).unwrap();
                let operand = chunk[1];
                match opcode {
                    Opcode::Adv => format!("adv {}", self.decode_combo_operand(operand)),
                    Opcode::Bxl => format!("bxl {}", operand),
                    Opcode::Bst => format!("bst {}", self.decode_combo_operand(operand)),
                    Opcode::Jnz => format!("jnz {}", operand),
                    Opcode::Bxc => "bxc".to_string(),
                    Opcode::Out => format!("out {}", self.decode_combo_operand(operand)),
                    Opcode::Bdv => format!("bdv {}", self.decode_combo_operand(operand)),
                    Opcode::Cdv => format!("cdv {}", self.decode_combo_operand(operand)),
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[allow(dead_code)]
    fn decode_combo_operand(&self, operand: usize) -> &str {
        match operand {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "A",
            5 => "B",
            6 => "C",
            7 => panic!("Reserved operand value"),
            _ => panic!("Invalid operand"),
        }
    }

    fn run(&mut self) -> Option<Vec<usize>> {
        let mut output = vec![];
        while self.ip < self.program.len() {
            let opcode = Opcode::from(self.program[self.ip])?;
            let operand = self.program[self.ip + 1];
            match opcode {
                Opcode::Adv => {
                    self.a >>= self.combo_operand(operand);
                    self.ip += 2;
                }
                Opcode::Bxl => {
                    self.b ^= operand;
                    self.ip += 2;
                }
                Opcode::Bst => {
                    self.b = self.combo_operand(operand) % 8;
                    self.ip += 2;
                }
                Opcode::Jnz => {
                    if self.a != 0 {
                        self.ip = operand;
                    } else {
                        self.ip += 2;
                    }
                }
                Opcode::Bxc => {
                    self.b ^= self.c;
                    self.ip += 2;
                }
                Opcode::Out => {
                    output.push(self.combo_operand(operand) % 8);
                    self.ip += 2;
                }
                Opcode::Bdv => {
                    self.b = self.a >> self.combo_operand(operand);
                    self.ip += 2;
                }
                Opcode::Cdv => {
                    self.c = self.a >> self.combo_operand(operand);
                    self.ip += 2;
                }
            }
        }
        Some(output)
    }

    fn combo_operand(&self, operand: usize) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Reserved operand value"),
            _ => panic!("Invalid operand"),
        }
    }
}

impl Opcode {
    fn from(value: usize) -> Option<Self> {
        match value {
            0 => Some(Self::Adv),
            1 => Some(Self::Bxl),
            2 => Some(Self::Bst),
            3 => Some(Self::Jnz),
            4 => Some(Self::Bxc),
            5 => Some(Self::Out),
            6 => Some(Self::Bdv),
            7 => Some(Self::Cdv),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 117440);
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
