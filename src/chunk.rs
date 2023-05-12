use std::iter::Enumerate;
use std::slice::Iter;

use crate::value::Value;

#[derive(FromPrimitive)]
pub enum OpCode {
    RETURN,
    CONSTANT,
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>
}

impl Chunk {
    pub fn disassemble(&self) {
        let mut instructions = self.code.iter().enumerate();
        while self.disassemble_instruction(&mut instructions).is_some() {}
    }
    fn disassemble_instruction(&self, instructions: &mut Enumerate<Iter<u8>>) -> Option<()> {
        let instruction = instructions.next()?;
        let opcode = num::FromPrimitive::from_u8(*instruction.1);
        let opcode = opcode.expect(format!("{} is not a valid opcode", instruction.1).as_str());

        match opcode {
            OpCode::RETURN => {
                println!("{:04} {}", instruction.0, "OP_RETURN");
            }
            OpCode::CONSTANT => {
                let constant_index = instructions.next().expect("Unexpected end of bytecode").1;
                let constant = self.constants.get(*constant_index as usize).expect("Constant index out of bounds of constant array");
                println!("{:04} {} {}", instruction.0, "OP_CONSTANT", constant);
            }
        }
        Some(())
    }
}
