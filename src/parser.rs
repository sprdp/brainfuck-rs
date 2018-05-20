use vm::Instruction;

fn get_ins(c: char) -> Instruction {
    match c {
        '>' =>  Instruction::incrPtr,
        '<' =>  Instruction::decrPtr,
        '+' =>  Instruction::incrByte,
        '-' =>  Instruction::decrByte,
        '.' =>  Instruction::writeByte,
        ',' =>  Instruction::readByte,
        '[' =>  Instruction::jumpStart,
        ']' =>  Instruction::jumpEnd,
        _   =>  Instruction::incrPtr
    }
}

pub fn parse_src(instructions: String) -> Vec<Instruction> {
    let mut instrs: Vec<Instruction> = Vec::new();
    for c in instructions.chars() {
        instrs.push(get_ins(c));
    }
    instrs
}