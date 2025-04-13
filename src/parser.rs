use crate::instruction::*;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::fs::read_to_string;

pub fn parse_optcode(s: &str) -> Op {
    match s.to_uppercase().as_str() {
        "DAT" => Op::Dat,
        "MOV" => Op::Mov,
        "ADD" => Op::Add,
        "SUB" => Op::Sub,
        "MUL" => Op::Mul,
        "DIV" => Op::Div,
        "MOD" => Op::Mod,
        "JMP" => Op::Jmp,
        "JMZ" => Op::Jmz,
        "JMN" => Op::Jmn,
        "DJN" => Op::Djn,
        "SPL" => Op::Spl,
        "CMP" => Op::Cmp,
        "SEQ" => Op::Seq,
        "SNE" => Op::Sne,
        "SLT" => Op::Slt,
        "LDP" => Op::Ldp,
        "STP" => Op::Stp,
        "NOP" => Op::Nop,
        _ => panic!("Invalid opcode: {}", s),
    }
}

pub fn parse_operand(s: &str) -> Operand {
    use AddressingMode::*;

    let mut chars = s.chars();
    let mode_char = chars.next().unwrap(); // Get the addressing mode character

    // Default addressing mode is Direct if no symbol (e.g. "10")
    let (addressing_mode, rest): (AddressingMode, String) = match mode_char {
        '#' => (Immediate, chars.collect()),
        '$' => (Direct, chars.collect()),
        '*' => (
            Indirect {
                position: Position::A,
                change: None,
            },
            chars.collect(),
        ),
        '@' => (
            Indirect {
                position: Position::B,
                change: None,
            },
            chars.collect(),
        ),
        '{' => (
            Indirect {
                position: Position::A,
                change: Some(IndirectModify {
                    kind: IndirectModifyKind::Decrement,
                    pre_post: PrePost::Pre,
                }),
            },
            chars.collect(),
        ),
        '<' => (
            Indirect {
                position: Position::B,
                change: Some(IndirectModify {
                    kind: IndirectModifyKind::Decrement,
                    pre_post: PrePost::Pre,
                }),
            },
            chars.collect(),
        ),
        '}' => (
            Indirect {
                position: Position::A,
                change: Some(IndirectModify {
                    kind: IndirectModifyKind::Increment,
                    pre_post: PrePost::Post,
                }),
            },
            chars.collect(),
        ),
        '>' => (
            Indirect {
                position: Position::B,
                change: Some(IndirectModify {
                    kind: IndirectModifyKind::Increment,
                    pre_post: PrePost::Post,
                }),
            },
            chars.collect(),
        ),
        // If no symbol, it's a direct addressing and mode_char is part of the number
        '0'..='9' | '+' | '-' => (Direct, std::iter::once(mode_char).chain(chars).collect()),
        _ => return todo!(),
    };

    let (sign, number_str) = match rest.chars().next() {
        Some('+') => (true, &rest[1..]),
        Some('-') => (false, &rest[1..]),
        _ => (true, &rest[..]),
    };

    let data = number_str.parse::<isize>().ok().unwrap();

    Operand {
        addressing_mode,
        data,
    }
}

pub fn parse_mod(s: &str) -> Modifier {
    match s.to_uppercase().as_str() {
        "A" => Modifier::A,
        "B" => Modifier::B,
        "AB" => Modifier::AB,
        "BA" => Modifier::BA,
        "F" => Modifier::F,
        "X" => Modifier::X,
        "I" => Modifier::I,
        _ => panic!("Invalid modifier: {}", s),
    }
}

pub fn parse_instruction(line: &str) -> Option<Instruction> {
    let parts: Vec<&str> = line
        .split(|c| c == ' ' || c == ',')
        .filter(|s| !s.trim().is_empty())
        .collect();

    if parts.is_empty() {
        return None;
    }

    // parse the

    let opsplit = parts[0].split(".").collect::<Vec<&str>>();

    let op = parse_optcode(opsplit[0]);

    let modifier = if opsplit.len() == 2 {
        parse_mod(opsplit[1])
    } else {
        match op {
            Op::Dat => Modifier::F,
            Op::Mov => Modifier::I,
            Op::Add => Modifier::AB,
            Op::Sub => Modifier::AB,
            Op::Mul => Modifier::AB,
            Op::Div => Modifier::AB,
            Op::Mod => Modifier::AB,
            Op::Jmp => Modifier::B,
            Op::Jmz => Modifier::B,
            Op::Jmn => Modifier::B,
            Op::Djn => Modifier::B,
            Op::Spl => Modifier::B,
            Op::Cmp => {
                // Handle CMP
                todo!()
            }
            Op::Seq => Modifier::I,
            Op::Sne => Modifier::I,
            Op::Slt => Modifier::B,
            Op::Ldp => {
                // Handle LDP
                todo!()
            }
            Op::Stp => {
                // Handle STP
                todo!()
            }
            Op::Nop => Modifier::F,
        }
    };

    // parse operands a, b
    let a = parse_operand(parts[1]);
    let b = parse_operand(parts[2]);

    return Some(Instruction { modifier, op, a, b });
}


pub fn parse_file(filename: &str) -> Vec<Instruction>{

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let m:Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut instructions = vec![];
    for line in m {
        match parse_instruction(line.as_str()) {
            Some(x) => {
                instructions.push(x)
            },
            None => {
                return vec![];
            }
        }
    }

    return instructions;
}