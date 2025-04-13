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
pub enum IndirectModifyKind {
    Increment,
    Decrement,
}

impl IndirectModifyKind {
    pub fn to_offset(self) -> isize {
        use IndirectModifyKind::*;
        match self {
            Increment => 1,
            Decrement => -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndirectModify {
    pub kind: IndirectModifyKind,
    pub pre_post: PrePost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrePost {
    Pre,
    Post,
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
    pub sign: bool,
    pub data: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instruction {
    pub modifier: Modifier,
    pub op: Op,
    pub a: Operand,
    pub b: Operand,
}
