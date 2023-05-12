mod chunk;
mod value;
extern crate num;
#[macro_use]
extern crate num_derive;

fn main() {
    // let code = vec![chunk::OpCode::RETURN];
    let mut chunk = chunk::Chunk {
        code: vec![],
        constants: vec![],
        lines: vec![]
    };

    chunk.constants.push(123.0);

    chunk.code.push(chunk::OpCode::RETURN as u8);
    chunk.code.push(chunk::OpCode::RETURN as u8);
    chunk.code.push(chunk::OpCode::RETURN as u8);
    chunk.code.push(chunk::OpCode::CONSTANT as u8);
    chunk.code.push(0); // constant index
    chunk.code.push(chunk::OpCode::RETURN as u8);
    chunk.code.push(chunk::OpCode::RETURN as u8);

    chunk.disassemble();
    println!("Hello, world!");
}
