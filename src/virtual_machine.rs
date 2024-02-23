use std::default;

use crate::{bytecode::{Chunk, OpCode, Value}, compiler::compile, debug::{disassemble_chunk, disassemble_instruction}};

pub struct VirtualMachine {
    pub chunk: Chunk,
    pub stack: Stack
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompilerError,
    InterpretRuntimeError
}

enum RuntimeError {
    TypeError(String)
}

impl VirtualMachine {
    pub fn interpret(&mut self, src: String) -> InterpretResult {

        if let Some(chunk) = compile(src) {
            self.chunk = chunk;
            let program = self.run(false);
            if let Err(error) = program {
                match error {
                    RuntimeError::TypeError(s) => println!("TYPE ERROR {}", s)
                }
                return InterpretResult::InterpretRuntimeError
            } else {
                return InterpretResult::InterpretOk
            }
        } 
        return InterpretResult::InterpretCompilerError

        

        
    }

    fn run(&mut self, execution_tracing: bool) -> Result<(), RuntimeError> {
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
                    return Ok(())
                }, OpCode::Constant(index) => {
                    if let Some(constant) = constants.get(*index) {
                        self.stack.push(constant.clone());
                    } else {
                        //TODO: Add better error handling
                        panic!("Cant find value")
                    }
                }, 
                OpCode::Negate =>  {
                    if let Value::Number(n) = self.stack.pop() {
                        self.stack.push(Value::Number(-n));
                    } else {
                        return Err(RuntimeError::TypeError(format!("Operand must be a number.")));
                    }
                    
                }
                OpCode::Add => {
                    let n2 = self.stack.pop();
                    let n1 = self.stack.pop();
                    self.stack.push(n1 + n2);
                },
                OpCode::Subtract => {
                    let n2 = self.stack.pop();
                    let n1 = self.stack.pop();
                    self.stack.push(n1 - n2);
                },
                OpCode::Multiply => {
                    let n2 = self.stack.pop();
                    let n1 = self.stack.pop();
                    self.stack.push(n1 * n2);
                },
                OpCode::Divide => {
                    let n2 = self.stack.pop();
                    let n1 = self.stack.pop();
                    self.stack.push(n1 / n2);
                },
            }
        }
        Ok(())
    }
}


// fn read_constant(constants: Vec<Constant>, i: usize) -> Constant {
    

// }

pub struct Stack {
    stack_vec: Vec<Value>,
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

    pub fn pop(&mut self) -> Value {
        return self.stack_vec.pop().unwrap();
    }

    pub fn push(&mut self, value: Value){
        return self.stack_vec.push(value);
    }

    pub fn display(&self) {
        print!("          ");
        for value in &self.stack_vec {
            print!("[{}]",value);
        }
    }
}

