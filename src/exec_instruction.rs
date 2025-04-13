use crate::{instruction::*, process::Game};

impl Game {
    fn eval_operand(&mut self, instruction_index: usize, operand: Operand) -> (Instruction, usize) {
        match operand.addressing_mode {
            AddressingMode::Immediate => (self.memory[0], 0),
            AddressingMode::Direct => (
                self.memory[instruction_index.wrapping_add_signed(operand.data)],
                instruction_index.wrapping_add_signed(operand.data),
            ),
            AddressingMode::Indirect {
                position,
                change: None,
            } => {
                let intermediate_instruction_index =
                    instruction_index.wrapping_add_signed(operand.data);
                let intermediate_instruction = self.memory[intermediate_instruction_index];
                let result_instruction_offset = match position {
                    Position::A => intermediate_instruction.a.data,
                    Position::B => intermediate_instruction.b.data,
                };
                let result_instruction_index =
                    intermediate_instruction_index.wrapping_add_signed(result_instruction_offset);
                let result_instruction = self.memory[result_instruction_index];
                (result_instruction, result_instruction_index)
            }
            AddressingMode::Indirect {
                position,
                change: Some(change),
            } => {
                let intermediate_instruction_index =
                    instruction_index.wrapping_add_signed(operand.data);
                if change.pre_post == PrePost::Pre {
                    // self.memory[intermediate_instruction_index] += change.kind.to_offset();
                    todo!()
                }
                let intermediate_instruction = self.memory[intermediate_instruction_index];
                let result_instruction_offset = match position {
                    Position::A => intermediate_instruction.a.data,
                    Position::B => intermediate_instruction.b.data,
                };
                let result_instruction_index = intermediate_instruction_index.wrapping_add_signed(result_instruction_offset);
                let result_instruction = self.memory[result_instruction_index];
                if change.pre_post == PrePost::Post {
                    // self.memory[intermediate_instruction_index] += change.kind.to_offset();
                    todo!()
                }
                todo!()
            }
        }
    }

    fn exec_instruction(&mut self, instruction: Instruction, ip: &mut usize) -> Option<()> {
        use Op::*;

        let instruction_index = *ip;
        let (a_instruction, a_instruction_index) =
            self.eval_operand(instruction_index, instruction.a);
        let (b_instruction, b_instruction_index) =
            self.eval_operand(instruction_index, instruction.b);

        match instruction.op {
            Dat => {
                return None;
            }
            Mov => {
                use Modifier::*;
                match instruction.modifier {
                    A => todo!(),
                    B => todo!(),
                    AB => todo!(),
                    BA => todo!(),
                    F => todo!(),
                    X => todo!(),
                    I => {
                        self.memory[b_instruction_index] = a_instruction;
                    }
                };
            }
            Add => {
                use Modifier::*;
                match instruction.modifier {
                    A => {
                        self.memory[b_instruction_index].a.data += a_instruction.a.data;
                    }
                    B => {
                        self.memory[b_instruction_index].b.data += a_instruction.b.data;
                    }
                    AB => {
                        self.memory[b_instruction_index].b.data += a_instruction.a.data;
                    }
                    BA => {
                        self.memory[b_instruction_index].a.data += a_instruction.b.data;
                    }
                    F | I => {
                        self.memory[b_instruction_index].a.data += a_instruction.a.data;
                        self.memory[b_instruction_index].b.data += a_instruction.b.data;
                    }
                    X => {
                        self.memory[b_instruction_index].a.data += a_instruction.b.data;
                        self.memory[b_instruction_index].b.data += a_instruction.a.data;
                    }
                };
            }
            Sub => {
                use Modifier::*;
                match instruction.modifier {
                    A => {
                        self.memory[b_instruction_index].a.data -= a_instruction.a.data;
                    }
                    B => {
                        self.memory[b_instruction_index].b.data -= a_instruction.b.data;
                    }
                    AB => {
                        self.memory[b_instruction_index].b.data -= a_instruction.a.data;
                    }
                    BA => {
                        self.memory[b_instruction_index].a.data -= a_instruction.b.data;
                    }
                    F | I => {
                        self.memory[b_instruction_index].a.data -= a_instruction.a.data;
                        self.memory[b_instruction_index].b.data -= a_instruction.b.data;
                    }
                    X => {
                        self.memory[b_instruction_index].a.data -= a_instruction.b.data;
                        self.memory[b_instruction_index].b.data -= a_instruction.a.data;
                    }
                };
            }
            Mul => {
                use Modifier::*;
                match instruction.modifier {
                    A => {
                        self.memory[b_instruction_index].a.data *= a_instruction.a.data;
                    }
                    B => {
                        self.memory[b_instruction_index].b.data *= a_instruction.b.data;
                    }
                    AB => {
                        self.memory[b_instruction_index].b.data *= a_instruction.a.data;
                    }
                    BA => {
                        self.memory[b_instruction_index].a.data *= a_instruction.b.data;
                    }
                    F | I => {
                        self.memory[b_instruction_index].a.data *= a_instruction.a.data;
                        self.memory[b_instruction_index].b.data *= a_instruction.b.data;
                    }
                    X => {
                        self.memory[b_instruction_index].a.data *= a_instruction.b.data;
                        self.memory[b_instruction_index].b.data *= a_instruction.a.data;
                    }
                };
            }
            Div => {
                use Modifier::*;
                match instruction.modifier {
                    A => {
                        self.memory[b_instruction_index].a.data /= a_instruction.a.data;
                    }
                    B => {
                        self.memory[b_instruction_index].b.data /= a_instruction.b.data;
                    }
                    AB => {
                        self.memory[b_instruction_index].b.data /= a_instruction.a.data;
                    }
                    BA => {
                        self.memory[b_instruction_index].a.data /= a_instruction.b.data;
                    }
                    F | I => {
                        self.memory[b_instruction_index].a.data /= a_instruction.a.data;
                        self.memory[b_instruction_index].b.data /= a_instruction.b.data;
                    }
                    X => {
                        self.memory[b_instruction_index].a.data /= a_instruction.b.data;
                        self.memory[b_instruction_index].b.data /= a_instruction.a.data;
                    }
                };
            }
            Mod => {
                use Modifier::*;
                match instruction.modifier {
                    A => {
                        self.memory[b_instruction_index].a.data %= a_instruction.a.data;
                    }
                    B => {
                        self.memory[b_instruction_index].b.data %= a_instruction.b.data;
                    }
                    AB => {
                        self.memory[b_instruction_index].b.data %= a_instruction.a.data;
                    }
                    BA => {
                        self.memory[b_instruction_index].a.data %= a_instruction.b.data;
                    }
                    F | I => {
                        self.memory[b_instruction_index].a.data %= a_instruction.a.data;
                        self.memory[b_instruction_index].b.data %= a_instruction.b.data;
                    }
                    X => {
                        self.memory[b_instruction_index].a.data %= a_instruction.b.data;
                        self.memory[b_instruction_index].b.data %= a_instruction.a.data;
                    }
                };
            }
            Jmp => {
                *ip = a_instruction_index;
            }
            Jmz => {
                // let b = match
            }
            Jmn => todo!(),
            Djn => todo!(),
            Spl => todo!(),
            Cmp => todo!(),
            Seq => todo!(),
            Sne => todo!(),
            Slt => todo!(),
            Ldp => todo!(),
            Stp => todo!(),
            Nop => todo!(),
        };
        Some(())
    }

    pub fn execute_one_cycle(&mut self) {
        let curr_player = &mut self.players.curr_player;
        let curr_process = &mut self.players.players[*curr_player].curr_process;
        // let ip = &mut self.players.players[*curr_player].processes[*curr_process];
        let ip = todo!();
        // let instruction = self.memory[ip];
        let instruction = todo!("Implement Jmn instruction");
        *curr_process += 1;
        *curr_player += 1;
        self.exec_instruction(instruction, ip);
    }
}
