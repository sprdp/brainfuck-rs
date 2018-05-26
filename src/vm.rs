use std::io::{self, Write, Read};


const CELLS: usize = 30000;

#[derive(PartialEq)]
pub enum Instruction {
    incrPtr,
    decrPtr,
    incrByte,
    decrByte,
    writeByte,
    readByte,
    jumpStart(usize),
    jumpEnd(usize),
}

struct MachineState {
    cellArray: [u8; CELLS],
    dataPtr: usize,
    stack: Vec<usize>,
}

impl MachineState {
    fn new() -> MachineState {
        MachineState {
            cellArray: [0; CELLS],
            dataPtr: 0,
            stack: vec![]
        }
    }
}

pub struct BFMachine {
    state: MachineState,
    input_stream: io::Stdin,
    output_stream: io::Stdout,
}

impl BFMachine {
    pub fn new() -> BFMachine {
        BFMachine {
            state: MachineState::new(),
            input_stream: io::stdin(),
            output_stream: io::stdout(),
        }
    }

    fn execute_instruction(&mut self, ins_type: &Instruction, ins_ptr: usize ) -> usize {
        let mut ret_ins = ins_ptr + 1;
        match ins_type {
            Instruction::incrPtr => self.state.dataPtr += 1;
            Instruction::decrPtr => self.state.dataPtr -= 1;
            Instruction::incrByte => self.state.cellArray[self.state.dataPtr] += 1;
            Instruction::decrByte => self.state.cellArray[self.state.dataPtr] -= 1;
            Instruction::writeByte => self.output_stream.write(&[self.state.cellArray[self.state.dataPtr]]);
            Instruction::readByte => self.input_stream.read(&mut [self.state.cellArray[self.state.dataPtr]]);
            Instruction::jumpStart(jump_addr) => {
                if self.state.cellArray[self.state.dataPtr] == 0 {
                    ret_ins = *jump_addr;
                }
            }
            Instruction::jumpEnd(jump_addr) => {
                if self.state.cellArray[self.state.dataPtr] != 0 {
                    ret_ins = *jump_addr;
                }
            }
        }
        ret_ins
    }

    pub fn process(&mut self, ins_list: Vec<Instruction>) {
        let mut ins_ptr: usize = 0;
        while ins_ptr < ins_list.len() {
            ins_ptr = self.execute_instruction(&ins_list[ins_ptr], ins_ptr);
        }
    }
}