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
}

impl Chunk {
    pub fn disassemble(&self) {
        let mut instructions = self.code.iter().enumerate();
        while disassemble_instruction(&mut instructions) {}
    }
}
fn disassemble_instruction(instructions: &mut Enumerate<Iter<u8>>) -> bool {
    loop {
        let instruction = instructions.next();
        match instruction {
            Some(instruction) => {
                match num::FromPrimitive::from_u8(*instruction.1).expect("Invalid opcode") {
                    OpCode::RETURN => {
                        println!("{:04} {}", instruction.0, "OP_RETURN");
                    }
                    OpCode::CONSTANT => {
                        println!("{:04} {}", instruction.0, "OP_CONSTANT");
                        let constant = instructions.next().expect("Unexpected end of bytecode");
                    }
                };
                break;
            }
            None => return false,
        }
    }
    return true;
}
