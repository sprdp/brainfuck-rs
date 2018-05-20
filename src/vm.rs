use std::io::{self, Write, Read};


const CELLS: usize = 30000;

pub enum Instruction {
    incrPtr,
    decrPtr,
    incrByte,
    decrByte,
    writeByte,
    readByte,
    jumpStart,
    jumpEnd,
}

struct MachineState {
    cellArray: [u8; CELLS],
    dataPtr: usize,
}

impl MachineState {
    fn new() -> MachineState {
        MachineState {
            cellArray: [0; CELLS],
            dataPtr: 0,
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

    fn execute_instruction(&mut self, ins_type: Instruction) {
        match ins_type {
            Instruction::incrPtr => {
                self.state.dataPtr += 1;
            }
            Instruction::decrPtr => {
                self.state.dataPtr -= 1;
            }
            Instruction::incrByte => {
                self.state.cellArray[self.state.dataPtr] += 1;
            }
            Instruction::decrByte => {
                self.state.cellArray[self.state.dataPtr] -= 1;
            }
            Instruction::writeByte => {
                self.output_stream.write(&[self.state.cellArray[self.state.dataPtr]]);
            }
            Instruction::readByte => {
                self.input_stream.read(&mut [self.state.cellArray[self.state.dataPtr]]);
            }
            Instruction::jumpStart => {
                self.state.dataPtr += 1;
            }
            Instruction::jumpEnd => {
                self.state.dataPtr += 1;
            }
        }
    }

    pub fn process(&mut self, ins_list: Vec<Instruction>) {
        for ins in ins_list{
            self.execute_instruction(ins)
        }
    }
}
