use std::convert::TryFrom;

pub struct IntcodeComputer<'a> {
    memory: Vec<isize>,
    pc: usize,
    in_fn: &'a mut dyn FnMut() -> isize,
    out_fn: &'a mut dyn FnMut(isize),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Intcode {
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    JNZ(Param, Param),
    JZ(Param, Param),
    LT(Param, Param, Param),
    EQ(Param, Param, Param),
    Input(Param),
    Output(Param),
    Halt,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Param {
    value: isize,
    mode: AddressingMode
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AddressingMode {
    Position,
    Immediate,
}

pub enum InvalidInstruction {
    MissingParams,
    NegativePositionalParam,
    Invalid(isize),
}

impl<'a> IntcodeComputer<'a> {
    pub fn new(program: &[isize], in_fn: &'a mut dyn FnMut() -> isize, out_fn: &'a mut dyn FnMut(isize)) -> Self {
        Self {
            memory: Vec::from(program),
            pc: 0,
            in_fn,
            out_fn,
        }
    }

    pub fn run(&mut self) {
        while let Ok(intcode) = Intcode::try_from(&self.memory[self.pc..]) {
            let mut jmp = false;
            match intcode {
                Intcode::Add(op1, op2, op3) => {
                    let (op1, op2) = (op1.fetch(self.memory()), op2.fetch(&self.memory()));
                    self.memory[op3.value as usize] = op1 + op2;
                },
                Intcode::Mul(op1, op2, op3) => {
                    let (op1, op2) = (op1.fetch(self.memory()), op2.fetch(&self.memory()));
                    self.memory[op3.value as usize] = op1 * op2;
                },
                Intcode::JNZ(op1, op2) => {
                    let (op1, op2) = (op1.fetch(self.memory()), op2.fetch(&self.memory()));
                    if op1 != 0 {
                        jmp = true;
                        self.pc = op2 as usize;
                    }
                }
                Intcode::JZ(op1, op2) => {
                    let (op1, op2) = (op1.fetch(self.memory()), op2.fetch(&self.memory()));
                    if op1 == 0 {
                        jmp = true;
                        self.pc = op2 as usize;
                    }
                }
                Intcode::LT(op1, op2, op3) => {
                    self.memory[op3.value as usize] = {
                        let (op1, op2) = (op1.fetch(self.memory()), op2.fetch(&self.memory()));
                        if op1 < op2 {
                            1
                        } else {
                            0
                        }
                    }
                }
                Intcode::EQ(op1, op2, op3) => {
                    self.memory[op3.value as usize] = {
                        let (op1, op2) = (op1.fetch(self.memory()), op2.fetch(&self.memory()));
                        if op1 == op2 {
                            1
                        } else {
                            0
                        }
                    }
                }
                Intcode::Input(op1) => {
                    self.memory[op1.value as usize] = (self.in_fn)();
                }
                Intcode::Output(op) => {
                    let op = op.fetch(self.memory());
                    (self.out_fn)(op);
                }
                Intcode::Halt => break,
            }
            if !jmp {
                self.pc += intcode.size();
            }
        }
    }

    pub fn memory(&self) -> &[isize] {
        &self.memory
    }
}

impl TryFrom<&[isize]> for Intcode {
    type Error = InvalidInstruction;

    fn try_from(input: &[isize]) -> Result<Self, Self::Error> {
        if input.is_empty() {
            return Err(InvalidInstruction::Invalid(0));
        }
        let mut v1 = input[0];
        let opcode = v1 % 100;
        v1 /= 100;
        let instruction = match opcode {
            op @ 1..=2 | op @ 7..=8 => {
                if input.len() < 4 {
                    return Err(InvalidInstruction::MissingParams)
                }
                let op1 = Param::new(input[1], v1 % 10)?;
                v1 /= 10;
                let op2 = Param::new(input[2], v1 % 10)?;
                v1 /= 10;
                let op3 = Param::new(input[3], v1 % 10)?;
                match op {
                    1 => Intcode::Add(op1, op2, op3),
                    2 => Intcode::Mul(op1, op2, op3),
                    7 => Intcode::LT(op1, op2, op3),
                    8 => Intcode::EQ(op1, op2, op3),
                    _ => unreachable!(),
                }
            }
            op @ 3..=4  => {
                if input.len() < 2 {
                    return Err(InvalidInstruction::MissingParams);
                }
                let op1 = Param::new(input[1], v1 % 10)?;
                match op {
                    3 => Intcode::Input(op1),
                    4 => Intcode::Output(op1),
                    _ => unreachable!(),
                }
            }
            op @ 5..=6 => {
                if input.len() < 3 {
                    return Err(InvalidInstruction::MissingParams);
                }
                let op1 = Param::new(input[1], v1 % 10)?;
                v1 /= 10;
                let op2 = Param::new(input[2], v1 % 10)?;
                match op {
                    5 => Intcode::JNZ(op1, op2),
                    6 => Intcode::JZ(op1, op2),
                    _ => unreachable!(),
                }
            }
            99 => Intcode::Halt,
            _ => return Err(InvalidInstruction::Invalid(input[0])),
        };
        Ok(instruction)
    }
}

impl Intcode {
    fn size(&self) -> usize {
        match self {
            Intcode::Add(_,_,_) => 4,
            Intcode::Mul(_,_,_) => 4,
            Intcode::JNZ(_, _) => 3,
            Intcode::JZ(_, _) => 3,
            Intcode::LT(_, _, _) => 4,
            Intcode::EQ(_, _, _) => 4,
            Intcode::Input(_) => 2,
            Intcode::Output(_) => 2,
            Intcode::Halt => 1,
        }
    }
}

impl Param {
    fn new(value: isize, mode: isize) -> Result<Self, InvalidInstruction> {
        let mode = AddressingMode::from(mode);
        if mode == AddressingMode::Position && value < 0 {
            return Err(InvalidInstruction::NegativePositionalParam);
        }
        Ok(Self {
            value,
            mode,
        })
    }
    fn fetch(&self, memory: &[isize]) -> isize {
        match self.mode {
            AddressingMode::Immediate => self.value,
            AddressingMode::Position => memory[self.value as usize],
        }
    }
}

impl From<isize> for AddressingMode {
    fn from(input: isize) -> Self {
        match input {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => unreachable!(),
        }
    }
}