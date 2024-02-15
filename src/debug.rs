use crate::bytecode::{Chunk, Constant, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    print!("== {} ==\n", name);
    for (op_code, line_number) in chunk.code.iter() {
        disassemble_instruction(op_code, line_number, &chunk.constant);
    }
}

pub fn disassemble_instruction(op_code: &OpCode, line: &u16, constants: &Vec<Constant>) {
    print!("{:<6}", line);
    match op_code {
        OpCode::Return => print!("OP_RETURN\n"),
        OpCode::Constant(c) => print!("OP_CONSTANT {}\n", constants[*c])
    }
}