use crate::bytecode::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    print!("== {} ==\n", name);

    for op_code in chunk.code.iter() {
        let (code, line) = op_code;
        print!("{:<6}", line);
        match code {
            OpCode::Return => print!("OP_RETURN\n"),
            OpCode::Constant(c) => print!("OP_CONSTANT {}\n", chunk.constant[*c])
        }
    }
}