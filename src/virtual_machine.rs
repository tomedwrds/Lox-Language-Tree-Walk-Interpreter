use std::{collections::HashMap, default, env::VarError};

use crate::{bytecode::{Chunk, OpCode, Value}, compiler::compile, debug::{disassemble_chunk, disassemble_instruction}};

pub struct VirtualMachine {
    pub chunk: Chunk,
    pub stack: Stack,
    globals: HashMap<String, Global>
}


struct Global {
    value: Value,
    is_const: bool
}
pub enum InterpretResult {
    InterpretOk,
    InterpretCompilerError,
    InterpretRuntimeError
}

enum RuntimeError {
    TypeError(String, usize),
    VarError(String, usize)
}

pub fn interpret_vm(src: String) -> InterpretResult {

    if let Some(chunk) = compile(src) {
        disassemble_chunk(&chunk, "Debug");
        let mut vm = VirtualMachine {
            chunk,
            stack: Stack::default(),
            globals: HashMap::new()
        };
        let program = vm.run(false);
        if let Err(error) = program {
            match error {
                RuntimeError::TypeError(s, l) => println!("TYPE ERROR on line {}: {}",l, s),
                RuntimeError::VarError(s, l) => println!("VAR ERROR on line {}: {}",l, s)
            }
            return InterpretResult::InterpretRuntimeError
        } else {
            return InterpretResult::InterpretOk
        }
    } 
    return InterpretResult::InterpretCompilerError
}

impl VirtualMachine {
    

    fn run(&mut self, execution_tracing: bool) -> Result<(), RuntimeError> {
        let code = &self.chunk.code;
        let constants = &self.chunk.constant;
        let mut ip = 0;
         
        while ip < code.len() {
            let (op_code, line_number) = &code[ip];
            if execution_tracing {
                self.stack.display();
                disassemble_instruction(op_code, line_number, constants); 
            }

            match op_code {
                OpCode::Return => {
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
                        return Err(RuntimeError::TypeError(format!("Operand must be a number."), *line_number));
                    }
                    
                }
                OpCode::Add => {
                    let n2 = self.stack.pop();
                    let n1 = self.stack.pop();
                    match (n1, n2) {
                        (Value::Number(n1), Value::Number(n2)) => self.stack.push(Value::Number(n1 + n2)),
                        (Value::String(s1), Value::String(s2)) => self.stack.push(Value::String(s1 + &s2)),
                        _ => return Err(RuntimeError::TypeError(format!("Operand must be either both string or number."), *line_number))
                    }
                },
                OpCode::Subtract => {
                    let n2 = self.stack.pop();
                    let n1 = self.stack.pop();
                    match (n1, n2) {
                        (Value::Number(n1), Value::Number(n2)) => self.stack.push(Value::Number(n1 - n2)),
                        _ => return Err(RuntimeError::TypeError(format!("Operand must be both number."), *line_number))
                    }
                },
                OpCode::Multiply => {
                    let n2 = self.stack.pop();
                    let n1 = self.stack.pop();
                    match (n1, n2) {
                        (Value::Number(n1), Value::Number(n2)) => self.stack.push(Value::Number(n1*n2)),
                        _ => return Err(RuntimeError::TypeError(format!("Operand must be both number."), *line_number))
                    }
                },
                OpCode::Divide => {
                    let n2 = self.stack.pop();
                    let n1 = self.stack.pop();
                    match (n1, n2) {
                        (Value::Number(n1), Value::Number(n2)) => self.stack.push(Value::Number(n1/n2)),
                        _ => return Err(RuntimeError::TypeError(format!("Operand must be both number."), *line_number))
                    }
                },
                OpCode::Not => {
                    if let Value::Bool(b) = self.stack.pop() {
                        self.stack.push(Value::Bool(!b));
                    } else {
                        return Err(RuntimeError::TypeError(format!("Can only negate boolean values."), *line_number))
                    }
                },
                OpCode::Equal => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(Value::Bool(a == b));
                },
                OpCode::Greater => {
                    let n2 = self.stack.pop();
                    let n1 = self.stack.pop();
                    match (n1, n2) {
                        (Value::Number(n1), Value::Number(n2)) => self.stack.push(Value::Bool(n1 > n2)),
                        _ => return Err(RuntimeError::TypeError(format!("Operand must be both number."), *line_number))
                    }
                },
                OpCode::Less => {
                    let n2 = self.stack.pop();
                    let n1 = self.stack.pop();
                    match (n1, n2) {
                        (Value::Number(n1), Value::Number(n2)) => self.stack.push(Value::Bool(n1 < n2)),
                        _ => return Err(RuntimeError::TypeError(format!("Operand must be both number."), *line_number))
                    }
                }, OpCode::Print => {
                    let v = self.stack.pop();
                    println!("{}",v);
                }, OpCode::Pop => {
                    self.stack.pop();
                },
                OpCode::DefineGlobal(index, is_const) => {
                    if let Some(Value::String(var_name)) = constants.get(*index) {
                        let var_value = self.stack.peek();
                        self.globals.insert(var_name.to_string(), Global {
                            value: var_value,
                            is_const: *is_const});
                        self.stack.pop();
                    } else {
                        //TODO: Add better error handling
                        panic!("Cant find var name")
                    }
                },
                OpCode::GetGlobal(name) => {
                    if let Some(global) = self.globals.get(name) {
                        self.stack.push(global.value.clone());
                    } else {
                        return Err(RuntimeError::VarError(format!("Undefined variable {}",name), *line_number))
                    }

                },
                OpCode::SetGlobal(name) => {
                    if let Some(global) = self.globals.get(name) {
                        if global.is_const {
                            return Err(RuntimeError::VarError(format!("Cannot reassign const variable {}",name), *line_number));
                        }
                        let value = self.stack.peek();
                        self.globals.insert(name.clone(), Global { value, is_const: global.is_const});
                    } else {
                        return Err(RuntimeError::VarError(format!("Undefined variable {}",name), *line_number))
                    }
                },
                OpCode::GetLocal(index) => {
                    if let Some(value) = self.stack.get(index) {
                        self.stack.push(value.clone());
                    } else {
                        return Err(RuntimeError::VarError(format!("Undefined variable"), *line_number))
                    }
                },
                OpCode::SetLocal(index) => {
                    let value = self.stack.peek();
                    self.stack.set(index, value);
                    
                },
                OpCode::JumpIfFalse(jump_size) => {
                    if let Value::Bool(condition) = self.stack.peek() {
                        if !condition {
                            ip += jump_size;
                        }
                    } else {
                        panic!("Attempt to evaluate no bool value");
                    }
                    
                },
                OpCode::Jump(jump_size) => {
                    ip += jump_size;
                }, 
                OpCode::Loop(jump_back) => {
                    ip -= jump_back;
                },
                OpCode::SwitchJump(jump_size) => {
                    let n1 = self.stack.pop();
                    let n2 = self.stack.peek();
                    if n1 != n2 {
                        ip += jump_size;
                    }
                },
                OpCode::Break => {
                    while true {
                        let (next_opcode, _) = &code[ip];
                        ip += 1;
                        if let OpCode::Loop(_) = next_opcode {
                            break;
                        }
                    }
                }
            }
            ip += 1;
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

    pub fn peek(&mut self) -> Value {
        //TODO improve this code
        return self.stack_vec.get(self.stack_vec.len() - 1).unwrap().clone();
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

    pub fn get(&mut self, index: &usize) -> Option<Value> {
        return self.stack_vec.get(*index).cloned();
    }

    pub fn set(&mut self, index: &usize, value: Value) {
        self.stack_vec[*index] = value;
    }
}

