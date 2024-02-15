use std::default;
use std::env;
use std::fs;

use bytecode::Chunk;
use bytecode::Constant;
use bytecode::OpCode;
use compiler::compile;
use debug::disassemble_chunk;
use virtual_machine::VirtualMachine;

use crate::interpreter::interpret;
mod scanner;
mod expr;
mod parser;
mod interpreter;
mod stmt;
mod enviroment;
mod lox_callable;
mod lox_instance;
mod bytecode;
mod debug;
mod virtual_machine;
mod compiler;

fn main() {
    let arg: Vec<String> = env::args().collect();
   
    if arg.len() == 1 {
        //run prompt left for now
    } else {
        let file = String::from("src/test.lox");
        run_file(&file);
    }
    let file = String::from("src/test.lox");
    let contents = fs::read_to_string(file)
        .expect("Error: file doesnt exist");
    compile(contents);
    // let chunk = Chunk {
    //     code: vec![
    //     (OpCode::Constant(0), 123),
    //     (OpCode::Constant(1), 123),
    //     (OpCode::Add, 123),
    //     (OpCode::Constant(2), 123),
    //     (OpCode::Divide, 123),
    //      (OpCode::Negate,123),  
    //      (OpCode::Return,123)],
    //     constant: vec![Constant::Number(1.2), Constant::Number(3.4), Constant::Number(5.6)]
    // };

    // let mut vm = VirtualMachine {
    //     chunk,
    //     stack: Default::default()
    // };
    // vm.interpret();

}

fn run_file(file_path: &String) {

}

// fn run_file_treewalk(file_path: &String) {
    
    
//     let scanner = scanner::scan(contents);
//     let parser = parser::parse(scanner.tokens);
//     let _interpreter = interpret(parser);
// }
