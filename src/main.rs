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
    };
    chunk.code.push(chunk::OpCode::RETURN as u8);
    chunk.code.push(chunk::OpCode::RETURN as u8);
    chunk.code.push(chunk::OpCode::RETURN as u8);
    chunk.code.push(chunk::OpCode::RETURN as u8);
    chunk.code.push(chunk::OpCode::RETURN as u8);

    chunk.disassemble();
    println!("Hello, world!");
}
