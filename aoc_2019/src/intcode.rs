use std::{
    borrow::Borrow,
    convert::TryFrom,
    error::Error,
    fmt::{self, Display},
    marker::PhantomData,
};

pub struct IntcodeComputer<I, U: Borrow<isize>, O> {
    memory: Vec<isize>,
    pc: usize,
    relative_base: isize,
    input: I,
    out_fn: O,
    pd: PhantomData<U>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Intcode {
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    Input(Param),
    Output(Param),
    JNZ(Param, Param),
    JZ(Param, Param),
    LT(Param, Param, Param),
    EQ(Param, Param, Param),
    RBO(Param),
    Halt,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Param {
    value: isize,
    mode: AddressingMode,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AddressingMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum InvalidInstruction {
    MissingParams,
    NegativePositionalParam,
    Invalid(isize),
    InvalidAddress,
}

impl<I: Iterator<Item = U>, U: Borrow<isize>, O> IntcodeComputer<I, U, O>
where
    O: FnMut(isize),
{
    pub fn new<T>(program: &[isize], input: T, out_fn: O) -> Self
    where
        T: IntoIterator<Item = U, IntoIter = I>,
    {
        let mut memory = Vec::from(program);
        memory.extend((0..program.len() * 10).map(|_| 0));
        Self {
            memory,
            pc: 0,
            relative_base: 0,
            input: input.into_iter(),
            out_fn,
            pd: PhantomData,
        }
    }

    pub fn run(&mut self) -> Result<(), InvalidInstruction> {
        while let Ok(intcode) = Intcode::try_from(&self.memory[self.pc..]) {
            let mut jmp = false;
            match intcode {
                Intcode::Add(op1, op2, op3) => {
                    let (op1, op2) = (self.eval_param(op1)?, self.eval_param(op2)?);
                    let loc = op3.memory_address(self.relative_base)?;
                    self.memory[loc] = op1 + op2;
                }
                Intcode::Mul(op1, op2, op3) => {
                    let (op1, op2) = (self.eval_param(op1)?, self.eval_param(op2)?);
                    let loc = op3.memory_address(self.relative_base)?;
                    self.memory[loc] = op1 * op2;
                }
                Intcode::JNZ(op1, op2) => {
                    let (op1, op2) = (self.eval_param(op1)?, self.eval_param(op2)?);
                    if op1 != 0 {
                        jmp = true;
                        self.pc = op2 as usize;
                    }
                }
                Intcode::JZ(op1, op2) => {
                    let (op1, op2) = (self.eval_param(op1)?, self.eval_param(op2)?);
                    if op1 == 0 {
                        jmp = true;
                        self.pc = op2 as usize;
                    }
                }
                Intcode::LT(op1, op2, op3) => {
                    let loc = op3.memory_address(self.relative_base)?;
                    self.memory[loc] = {
                        let (op1, op2) = (self.eval_param(op1)?, self.eval_param(op2)?);
                        if op1 < op2 {
                            1
                        } else {
                            0
                        }
                    }
                }
                Intcode::EQ(op1, op2, op3) => {
                    let loc = op3.memory_address(self.relative_base)?;
                    self.memory[loc] = {
                        let op1 = self.eval_param(op1)?;
                        let op2 = self.eval_param(op2)?;
                        if op1 == op2 {
                            1
                        } else {
                            0
                        }
                    }
                }
                Intcode::Input(op1) => {
                    let loc = op1.memory_address(self.relative_base)?;
                    self.memory[loc] = *self.input.next().unwrap().borrow();
                }
                Intcode::Output(op) => {
                    let op = self.eval_param(op)?;
                    (self.out_fn)(op);
                }
                Intcode::RBO(op) => {
                    let op = self.eval_param(op)?;
                    self.relative_base += op;
                }
                Intcode::Halt => break,
            }
            if !jmp {
                self.pc += intcode.size();
            }
        }
        Ok(())
    }

    pub fn memory(&self) -> &[isize] {
        &self.memory
    }

    pub fn eval_param(&self, param: Param) -> Result<isize, InvalidInstruction> {
        match param.mode {
            AddressingMode::Immediate => Ok(param.value),
            AddressingMode::Position => Ok(self.memory[param.value as usize]),
            AddressingMode::Relative => {
                if self.relative_base + param.value >= 0 {
                    Ok(self.memory[(self.relative_base + param.value) as usize])
                } else {
                    Err(InvalidInstruction::InvalidAddress)
                }
            }
        }
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
                    return Err(InvalidInstruction::MissingParams);
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
            op @ 3..=4 | op @ 9 => {
                if input.len() < 2 {
                    return Err(InvalidInstruction::MissingParams);
                }
                let op1 = Param::new(input[1], v1 % 10)?;
                match op {
                    3 => Intcode::Input(op1),
                    4 => Intcode::Output(op1),
                    9 => Intcode::RBO(op1),
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
            Intcode::Add(_, _, _) => 4,
            Intcode::Mul(_, _, _) => 4,
            Intcode::JNZ(_, _) => 3,
            Intcode::JZ(_, _) => 3,
            Intcode::LT(_, _, _) => 4,
            Intcode::EQ(_, _, _) => 4,
            Intcode::Input(_) => 2,
            Intcode::Output(_) => 2,
            Intcode::RBO(_) => 2,
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
        Ok(Self { value, mode })
    }

    fn memory_address(&self, base_offset: isize) -> Result<usize, InvalidInstruction> {
        match self.mode {
            AddressingMode::Position => Ok(self.value as usize),
            AddressingMode::Relative => {
                if self.value + base_offset < 0 {
                    return Err(InvalidInstruction::InvalidAddress);
                }
                Ok((self.value + base_offset) as usize)
            }
            _ => Err(InvalidInstruction::InvalidAddress),
        }
    }
}

impl From<isize> for AddressingMode {
    fn from(input: isize) -> Self {
        match input {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => unreachable!(),
        }
    }
}

impl Display for InvalidInstruction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidInstruction::InvalidAddress => {
                write!(fmt, "Attempted to access invalid memory.")
            }
            InvalidInstruction::MissingParams => {
                write!(fmt, "Instruction missing one or more parameters.")
            }
            _ => write!(fmt, "Invalid Instruction"),
        }
    }
}

impl Error for InvalidInstruction {}
