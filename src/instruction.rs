use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Dat,
    Mov,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Jmp,
    Jmz,
    Jmn,
    Djn,
    Spl,
    Cmp,
    Seq,
    Sne,
    Slt,
    Ldp,
    Stp,
    Nop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Position {
    A,
    B,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IndirectModify {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressingMode {
    Immediate,
    Direct,
    Indirect {
        position: Position,
        change: Option<IndirectModify>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Operand {
    pub addressing_mode: AddressingMode,
    pub data: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instruction {
    pub modifier: Modifier,
    pub op: Op,
    pub a: Operand,
    pub b: Operand,
}

pub fn dat_zero() -> Instruction {
    Instruction {
        modifier: Modifier::I,
        op: Op::Dat,
        a: Operand {
            addressing_mode: AddressingMode::Immediate,
            data: 0,
        },
        b: Operand {
            addressing_mode: AddressingMode::Immediate,
            data: 0,
        },
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self.op {
            Op::Dat => "dat",
            Op::Mov => "mov",
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Mul => "mul",
            Op::Div => "div",
            Op::Mod => "mod",
            Op::Jmp => "jmp",
            Op::Jmz => "jmz",
            Op::Jmn => "jmn",
            Op::Djn => "djn",
            Op::Spl => "spl",
            Op::Cmp => "cmp",
            Op::Seq => "seq",
            Op::Sne => "sne",
            Op::Slt => "slt",
            Op::Ldp => "ldp",
            Op::Stp => "stp",
            Op::Nop => "nop",
        };

        let txt2 = match self.modifier {
            Modifier::A => "a",
            Modifier::B => "b",
            Modifier::AB => "ab",
            Modifier::BA => "ba",
            Modifier::F => "f",
            Modifier::X => "x",
            Modifier::I => "i",
        };

        let txt3 = match self.a.addressing_mode {
            AddressingMode::Immediate => "#",
            AddressingMode::Direct => "$",
            AddressingMode::Indirect {
                position: Position::A,
                change: None,
            } => "*",
            AddressingMode::Indirect {
                position: Position::B,
                change: None,
            } => "@",
            AddressingMode::Indirect {
                position: Position::A,
                change: Some(IndirectModify::Decrement),
            } => "{",
            AddressingMode::Indirect {
                position: Position::B,
                change: Some(IndirectModify::Decrement),
            } => "<",
            AddressingMode::Indirect {
                position: Position::A,
                change: Some(IndirectModify::Increment),
            } => "}",
            AddressingMode::Indirect {
                position: Position::B,
                change: Some(IndirectModify::Increment),
            } => ">",
        };

        let txt4 = match self.b.addressing_mode {
            AddressingMode::Immediate => "#",
            AddressingMode::Direct => "$",
            AddressingMode::Indirect {
                position: Position::A,
                change: None,
            } => "*",
            AddressingMode::Indirect {
                position: Position::B,
                change: None,
            } => "@",
            AddressingMode::Indirect {
                position: Position::A,
                change: Some(IndirectModify::Decrement),
            } => "{",
            AddressingMode::Indirect {
                position: Position::B,
                change: Some(IndirectModify::Decrement),
            } => "<",
            AddressingMode::Indirect {
                position: Position::A,
                change: Some(IndirectModify::Increment),
            } => "}",
            AddressingMode::Indirect {
                position: Position::B,
                change: Some(IndirectModify::Increment),
            } => ">",
        };

        write!(
            f,
            "{}.{} {}{} {}{}",
            txt, txt2, txt3, self.a.data, txt4, self.b.data
        )
    }
}
