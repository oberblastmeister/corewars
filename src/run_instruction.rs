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
                if change == IndirectModify::Decrement {
                    // self.memory[intermediate_instruction_index] += change.kind.to_offset();
                    todo!()
                }
                let intermediate_instruction = self.memory[intermediate_instruction_index];
                let result_instruction_offset = match position {
                    Position::A => intermediate_instruction.a.data,
                    Position::B => intermediate_instruction.b.data,
                };
                let result_instruction_index =
                    intermediate_instruction_index.wrapping_add_signed(result_instruction_offset);
                let result_instruction = self.memory[result_instruction_index];
                if change == IndirectModify::Increment {
                    // self.memory[intermediate_instruction_index] += change.kind.to_offset();
                    todo!()
                }
                todo!()
            }
        }
    }

    fn run_instruction(&mut self, instruction: Instruction) -> Option<()> {
        use Op::*;

        let instruction_index = *self.get_current_ip();
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
                *self.get_current_ip() += 1;
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
                *self.get_current_ip() += 1;
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
                *self.get_current_ip() += 1;
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
                *self.get_current_ip() += 1;
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
                *self.get_current_ip() += 1;
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
                *self.get_current_ip() += 1;
            }
            Jmp => {
                *self.get_current_ip() = a_instruction_index;
            }
            Jmz => {
                todo!()
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

    pub fn get_current_player(&mut self) -> &mut usize {
        &mut self.players.curr_player
    }
    pub fn get_current_process(&mut self) -> &mut usize {
        let curr_player = self.players.curr_player;
        &mut self.players.players[curr_player].curr_process
    }

    pub fn get_current_ip(&mut self) -> &mut usize {
        let curr_player = self.players.curr_player;
        let curr_process = *self.get_current_process();
        &mut self.players.players[curr_player].processes[curr_process]
    }

    pub fn run_cycle(&mut self) {
        let ip = *self.get_current_ip();
        let instruction = self.memory[ip];
        self.run_instruction(instruction);
        *self.get_current_process() += 1;
        *self.get_current_player() += 1;
    }
}
