use crate::bytecode::{Chunk, Value, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    print!("== {} ==\n", name);
    for (op_code, line_number) in chunk.code.iter() {
        disassemble_instruction(op_code, line_number, &chunk.constant);
    }
}

pub fn disassemble_instruction(op_code: &OpCode, line: &usize, constants: &Vec<Value>) {
    print!("{:<6}", line);
    match op_code {
        OpCode::Return => print!("OP_RETURN\n"),
        OpCode::Constant(c) => print!("OP_CONSTANT {}\n", constants[*c]),
        OpCode::Negate => print!("OP_NEGATE\n"),
        OpCode::Add => print!("OP_ADD\n"),
        OpCode::Subtract => print!("OP_SUBTRACT\n"),
        OpCode::Multiply => print!("OP_MULTIPLY\n"),
        OpCode::Divide => print!("OP_DIVIDE\n"),
        OpCode::Not => print!("OP_NEGATE\n"),
        OpCode::Less => print!("OP_LESS\n"),
        OpCode::Equal => print!("OP_EQUAL\n"),
        OpCode::Greater => print!("OP_GREATER\n"),
        OpCode::Print => print!("OP_PRINT\n"),
        OpCode::Pop => print!("OP_POP\n"),
        OpCode::DefineGlobal(c,b) => print!("OP_DEFINE_GLOBAL {} {}\n", constants[*c], if *b { "CONST" } else { "VAR" }),
        OpCode::GetGlobal(n) => print!("OP_GET_GLOBAL {}\n",n),
        OpCode::SetGlobal(n) => print!("OP_SET_GLOBAL {}\n",n),
        OpCode::GetLocal(n) => print!("OP_GET_LOCAL {}\n",n),
        OpCode::SetLocal(n) => print!("OP_SET_LOCAL {}\n",n),
        OpCode::JumpIfFalse(n) => print!("OP_JUMP_IF_FALSE {}\n",n),


    }
}