static CELLS: i32 = 50000;

enum Instruction {
    incrPtr,
    decrPtr,
    incrByte,
    decrByte,
    writeByte,
    readByte,
    jumpStart,
    jumpEnd

}

struct Cell {
    value : int
};

impl Cell {
    fn increment(&self) {
        self.value++;
    }

    fn decrement(&self) {
        self.value++;
    }
}


struct MachineState {
    cellArray: [char; CELLS],
    dataPtr: &cellArray[0]
};


struct BFMachine {
    state: MachineState,
    inputStream: 
    outputStream:

};

impl BFMachine {
    executeInstruction(&self, insType: Instruction) {
        

         match Instruction {
            Instruction::incrPtr => {
                self.state.dataPtr += 1;
            }
            Instruction::Nickel => 5,
            Instruction::Dime => 10,
            Instruction::Quarter => 25,
        }
    }
}