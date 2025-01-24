use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing register {0} line")]
    MissingRegisterLine(char),
    #[error("Invalid register {0} prefix")]
    InvalidRegisterPrefix(char),
    #[error("Missing blank line")]
    MissingBlankLine,
    #[error("Missing program line")]
    MissingProgramLine,
    #[error("Invalid program prefix")]
    InvalidProgramPrefix,
    #[error("Failed to parse number: {0}")]
    FailedToParseNumber(#[from] std::num::ParseIntError),
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(usize),
    #[error("Reserved operand value")]
    ReservedOperandValue,
    #[error("Invalid operand")]
    InvalidOperand,
}

pub fn solve(input: &str) -> Result<String, Error> {
    let mut computer = Computer::parse(input)?;

    #[cfg(debug_assertions)]
    println!("{}", computer.disassemble());

    Ok(run_program(&computer.program, &mut computer.registers)
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(","))
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
    fn from(value: usize) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Literal(0)),
            1 => Ok(Self::Literal(1)),
            2 => Ok(Self::Literal(2)),
            3 => Ok(Self::Literal(3)),
            4 => Ok(Self::RegisterA),
            5 => Ok(Self::RegisterB),
            6 => Ok(Self::RegisterC),
            7 => Err(Error::ReservedOperandValue),
            _ => Err(Error::InvalidOperand),
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
            _ => "Invalid operand",
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
    fn from(opcode: usize, operand: usize) -> Result<Self, Error> {
        match opcode {
            0 => Ok(Self::Adv(ComboOperand::from(operand)?)),
            1 => Ok(Self::Bxl(operand)),
            2 => Ok(Self::Bst(ComboOperand::from(operand)?)),
            3 => Ok(Self::Jnz(operand / 2)),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out(ComboOperand::from(operand)?)),
            6 => Ok(Self::Bdv(ComboOperand::from(operand)?)),
            7 => Ok(Self::Cdv(ComboOperand::from(operand)?)),
            _ => Err(Error::InvalidOpcode(opcode)),
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
    program: Vec<Instruction>,
}

impl Computer {
    fn parse(input: &str) -> Result<Self, Error> {
        let mut lines = input.lines();
        let a = lines
            .next()
            .ok_or(Error::MissingRegisterLine('A'))?
            .strip_prefix("Register A: ")
            .ok_or(Error::InvalidRegisterPrefix('A'))?
            .parse()?;
        let b = lines
            .next()
            .ok_or(Error::MissingRegisterLine('B'))?
            .strip_prefix("Register B: ")
            .ok_or(Error::InvalidRegisterPrefix('B'))?
            .parse()?;
        let c = lines
            .next()
            .ok_or(Error::MissingRegisterLine('C'))?
            .strip_prefix("Register C: ")
            .ok_or(Error::InvalidRegisterPrefix('C'))?
            .parse()?;
        lines.next().ok_or(Error::MissingBlankLine)?;
        let raw_program = lines
            .next()
            .ok_or(Error::MissingProgramLine)?
            .strip_prefix("Program: ")
            .ok_or(Error::InvalidProgramPrefix)?
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, ParseIntError>>()?;
        let program = raw_program
            .chunks_exact(2)
            .map(|chunk| Instruction::from(chunk[0], chunk[1]))
            .collect::<Result<_, _>>()?;
        Ok(Self {
            registers: Registers { a, b, c, ip: 0 },
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

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
