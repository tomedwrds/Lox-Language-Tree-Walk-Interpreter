use crate::{bytecode::{Chunk, Constant, OpCode}, debug::disassemble_instruction,};

pub struct VirtualMachine {
    pub chunk: Chunk,
    pub stack: Stack
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompilerError,
    InterpretRuntimeError
}

impl VirtualMachine {
    pub fn interpret(&mut self) -> InterpretResult {
        return self.run(false);
    }

    fn run(&mut self, execution_tracing: bool) -> InterpretResult {
        let code = &self.chunk.code;
        let constants = &self.chunk.constant;

        for (op_code, line_number) in code {

            if execution_tracing {
                self.stack.display();
                disassemble_instruction(op_code, line_number, constants); 
            }

            match op_code {
                OpCode::Return => {
                    print!("{}\n", self.stack.pop());
                    return InterpretResult::InterpretOk
                },
                OpCode::Constant(index) => {
                    if let Some(constant) = constants.get(*index) {
                        self.stack.push(constant.clone());
                    } else {
                        //TODO: Add better error handling
                        return InterpretResult::InterpretRuntimeError
                    }
                }
            }
        }
        return InterpretResult::InterpretOk;
    }
}


// fn read_constant(constants: Vec<Constant>, i: usize) -> Constant {
    

// }

pub struct Stack {
    stack_vec: Vec<Constant>,
}

impl Default for Stack {
    fn default() -> Self {
        Stack {
            stack_vec: vec![],
        }
    }
}

impl Stack {
    pub fn reset(&mut self) {
        self.stack_vec = vec![]
    }

    pub fn pop(&mut self) -> Constant {
        return self.stack_vec.pop().unwrap();
    }

    pub fn push(&mut self, value: Constant){
        return self.stack_vec.push(value);
    }

    pub fn display(&self) {
        print!("          ");
        for value in &self.stack_vec {
            print!("[{}]",value);
        }
    }
}