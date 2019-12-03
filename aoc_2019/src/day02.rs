use std::convert::TryFrom;

enum Opcode {
    Addition(usize, usize, usize),
    Multiplication(usize, usize, usize),
    Halt,
}

impl Opcode {
    fn size(&self) -> usize {
        match self {
            Opcode::Addition(_, _, _) => 4,
            Opcode::Multiplication(_, _, _) => 4,
            Opcode::Halt => 1,
        }
    }
}

impl TryFrom<&[usize]> for Opcode {
    type Error = &'static str;
    fn try_from(value: &[usize]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Invalid Input");
        }

        match value[0] {
            1 => {
                if value.len() < 4 {
                    return Err("Missing Params");
                }
                Ok(Opcode::Addition(value[1], value[2], value[3]))
            }
            2 => {
                if value.len() < 4 {
                    return Err("Missing Params");
                }
                Ok(Opcode::Multiplication(value[1], value[2], value[3]))
            }
            99 => Ok(Opcode::Halt),
            _ => Err("Invalid Opcode"),
        }
    }
}

#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<usize> {
    let mut input: Vec<usize> = input
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    input[1] = 12;
    input[2] = 2;
    input
}

#[aoc(day2, part1)]
fn part1(input: &[usize]) -> usize {
    let mut memory: Vec<usize> = Vec::from(input);
    let mut pc = 0;

    // NOTE: This ignores malformed input
    while let Ok(op) = Opcode::try_from(&memory[pc..]) {
        match op {
            Opcode::Addition(op1, op2, loc) => memory[loc] = memory[op1] + memory[op2],
            Opcode::Multiplication(op1, op2, loc) => memory[loc] = memory[op1] * memory[op2],
            Opcode::Halt => {
                break;
            }
        }
        pc += op.size();
    }
    memory[0]
}

#[aoc(day2, part2)]
fn part2(input: &[usize]) -> usize {
    let mut input = Vec::from(input);
    for noun in 0..=99 {
        for verb in 0..=99 {
            input[1] = noun;
            input[2] = verb;
            if part1(&input) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn d02_p1() {
        let in1 = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(part1(&in1), 30);
    }
}
