use vm::Instruction;

pub fn parse_src(instructions: String) -> Vec<Instruction> {
    let mut instrs: Vec<Instruction> = Vec::new();
    let mut tokenCount: usize = 0;
    let mut jump_stack = vec![];

    for (i, c) in instructions.chars().enumerate() {
        match c {
            '>' => {
                instrs.push(Instruction::incrPtr);
                tokenCount += 1;
            }
            '<' => {
                instrs.push(Instruction::decrPtr);
                tokenCount += 1;
            }
            '+' => {
                instrs.push(Instruction::incrByte);
                tokenCount += 1;
            }
            '-' => {
                instrs.push(Instruction::decrByte);
                tokenCount += 1;
            }
            '.' => {
                instrs.push(Instruction::writeByte);
                tokenCount += 1;
            }
            ',' => {
                instrs.push(Instruction::readByte);
                tokenCount += 1;
            }
            '[' => {
                // Reserve space on vector with dummy instruction
                instrs.push(Instruction::jumpStart(0));
                jump_stack.push(tokenCount);
                tokenCount += 1;
            }
            ']' => {
                match jump_stack.pop() {
                    Some(jump_addr) => {
                        instrs.push(Instruction::jumpEnd(jump_addr + 1));
                        instrs[jump_addr] = Instruction::jumpStart(tokenCount + 1);
                    }
                    _ => panic!("Parsing error"),
                };
                tokenCount += 1;
            }
            _ => {
                // No op. Ignore all other tokens
            }
        }
    }
    instrs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() {
        let result_vec = parse_src(String::from("x[t>a<+.-,2]3"));
        assert_eq!(result_vec.len(), 8);
    }

    #[test]
    fn parse_test_full() {
        let result_vec = parse_src(String::from("x[t>a<+.-,2]3"));

        assert!(result_vec[0] == Instruction::jumpStart(8));
        assert!(result_vec[1] == Instruction::incrPtr);
        assert!(result_vec[2] == Instruction::decrPtr);
        assert!(result_vec[3] == Instruction::incrByte);
        assert!(result_vec[4] == Instruction::writeByte);
        assert!(result_vec[5] == Instruction::decrByte);
        assert!(result_vec[6] == Instruction::readByte);
        assert!(result_vec[7] == Instruction::jumpEnd(1));
    }
}
