pub fn solve(input: &str) -> String {
    let mut computer = Computer::parse(input).expect("Unable to parse input");
    computer
        .run()
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    program: Vec<usize>,
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

    fn run(&mut self) -> Vec<usize> {
        let mut output = vec![];
        while self.ip < self.program.len() {
            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            match opcode {
                0 => {
                    // adv
                    self.a >>= self.combo_operand(operand);
                    self.ip += 2;
                }
                1 => {
                    // bxl
                    self.b ^= operand;
                    self.ip += 2;
                }
                2 => {
                    // bst
                    self.b = self.combo_operand(operand) % 8;
                    self.ip += 2;
                }
                3 => {
                    // jnz
                    if self.a != 0 {
                        self.ip = operand;
                    } else {
                        self.ip += 2;
                    }
                }
                4 => {
                    // bxc
                    self.b ^= self.c;
                    self.ip += 2;
                }
                5 => {
                    // out
                    output.push(self.combo_operand(operand) % 8);
                    self.ip += 2;
                }
                6 => {
                    // bdv
                    self.b = self.a >> self.combo_operand(operand);
                    self.ip += 2;
                }
                7 => {
                    // cdv
                    self.c = self.a >> self.combo_operand(operand);
                    self.ip += 2;
                }
                _ => {
                    panic!("Invalid opcode");
                }
            }
        }
        output
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
