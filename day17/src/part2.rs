pub fn solve(input: &str) -> usize {
    let computer = Computer::parse(input).expect("Unable to parse input");

    #[cfg(debug_assertions)]
    println!("{}", computer.disassemble());

    computer
        .raw_program
        .iter()
        .rev()
        .fold(vec![0], |acc, next| {
            acc.into_iter()
                .flat_map(|a| (0..8).map(move |n| a << 3 | n))
                .filter(|&a| {
                    let mut registers = Registers {
                        a,
                        ..computer.registers
                    };
                    let output = run_program(&computer.program, &mut registers);
                    output.first() == Some(next)
                })
                .collect()
        })
        .first()
        .copied()
        .expect("Not found")
}

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
}

#[derive(Debug, Clone, Copy)]
enum ComboOperand {
    Literal(usize),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl ComboOperand {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Literal(0),
            1 => Self::Literal(1),
            2 => Self::Literal(2),
            3 => Self::Literal(3),
            4 => Self::RegisterA,
            5 => Self::RegisterB,
            6 => Self::RegisterC,
            7 => panic!("Reserved operand value"),
            _ => panic!("Invalid operand"),
        }
    }

    fn value(&self, registers: &Registers) -> usize {
        match self {
            Self::Literal(value) => *value,
            Self::RegisterA => registers.a,
            Self::RegisterB => registers.b,
            Self::RegisterC => registers.c,
        }
    }

    #[allow(dead_code)]
    fn disassemble(&self) -> &str {
        match self {
            Self::Literal(0) => "0",
            Self::Literal(1) => "1",
            Self::Literal(2) => "2",
            Self::Literal(3) => "3",
            Self::RegisterA => "A",
            Self::RegisterB => "B",
            Self::RegisterC => "C",
            _ => panic!("Invalid operand"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(usize),
    Bst(ComboOperand),
    Jnz(usize),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl Instruction {
    fn from(opcode: usize, operand: usize) -> Self {
        match opcode {
            0 => Self::Adv(ComboOperand::from(operand)),
            1 => Self::Bxl(operand),
            2 => Self::Bst(ComboOperand::from(operand)),
            3 => Self::Jnz(operand / 2),
            4 => Self::Bxc,
            5 => Self::Out(ComboOperand::from(operand)),
            6 => Self::Bdv(ComboOperand::from(operand)),
            7 => Self::Cdv(ComboOperand::from(operand)),
            _ => panic!("Invalid opcode"),
        }
    }

    fn apply(&self, registers: &mut Registers, output: &mut Vec<usize>) {
        match self {
            Self::Adv(operand) => {
                registers.a >>= operand.value(registers);
                registers.ip += 1;
            }
            Self::Bxl(value) => {
                registers.b ^= *value;
                registers.ip += 1;
            }
            Self::Bst(operand) => {
                registers.b = operand.value(registers) % 8;
                registers.ip += 1;
            }
            Self::Jnz(value) => {
                if registers.a != 0 {
                    registers.ip = *value;
                } else {
                    registers.ip += 1;
                }
            }
            Self::Bxc => {
                registers.b ^= registers.c;
                registers.ip += 1;
            }
            Self::Out(operand) => {
                output.push(operand.value(registers) % 8);
                registers.ip += 1;
            }
            Self::Bdv(operand) => {
                registers.b = registers.a >> operand.value(registers);
                registers.ip += 1;
            }
            Self::Cdv(operand) => {
                registers.c = registers.a >> operand.value(registers);
                registers.ip += 1;
            }
        }
    }

    #[allow(dead_code)]
    fn disassemble(&self) -> String {
        match self {
            Self::Adv(operand) => format!(
                "adv {0}\t// Shift value in register A right by {0}",
                operand.disassemble()
            ),
            Self::Bxl(value) => format!(
                "bxl {0}\t// Bitwise XOR value in register B with {0}",
                value
            ),
            Self::Bst(operand) => format!(
                "bst {0}\t// Write the value of {0} modulo 8 to register B",
                operand.disassemble()
            ),
            Self::Jnz(value) => format!(
                "jnz {0}\t// Jump to instruction {0:03} if value in register A is not zero",
                value * 2
            ),
            Self::Bxc => {
                "bxc  \t// Bitwise XOR values in registers B and C, storing the result in register B"
                    .to_string()
            }
            Self::Out(operand) => format!(
                "out {0}\t// Output value of {0} modulo 8",
                operand.disassemble()
            ),
            Self::Bdv(operand) => format!(
                "bdv {0}\t// Write value in register A to register B shifted right by {0}",
                operand.disassemble()
            ),
            Self::Cdv(operand) => format!(
                "cdv {0}\t// Write value in register A to register C shifted right by {0}",
                 operand.disassemble()
            ),
        }
    }
}

struct Computer {
    registers: Registers,
    raw_program: Vec<usize>,
    program: Vec<Instruction>,
}

impl Computer {
    fn parse(input: &str) -> Option<Self> {
        let mut lines = input.lines();
        let a = lines.next()?.strip_prefix("Register A: ")?.parse().ok()?;
        let b = lines.next()?.strip_prefix("Register B: ")?.parse().ok()?;
        let c = lines.next()?.strip_prefix("Register C: ")?.parse().ok()?;
        lines.next()?;
        let raw_program = lines
            .next()?
            .strip_prefix("Program: ")?
            .split(',')
            .flat_map(|s| s.parse())
            .collect::<Vec<_>>();
        let program = raw_program
            .chunks_exact(2)
            .map(|chunk| Instruction::from(chunk[0], chunk[1]))
            .collect();
        Some(Self {
            registers: Registers { a, b, c, ip: 0 },
            raw_program,
            program,
        })
    }

    #[allow(dead_code)]
    fn disassemble(&self) -> String {
        self.program
            .iter()
            .enumerate()
            .map(|(index, instruction)| format!("{:03}: {}", index * 2, instruction.disassemble()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn run_program(program: &[Instruction], registers: &mut Registers) -> Vec<usize> {
    let mut output = vec![];
    while let Some(instruction) = program.get(registers.ip) {
        instruction.apply(registers, &mut output);
    }
    output
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
