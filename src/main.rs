use std::env;
use std::fs;

use bytecode::Chunk;
use bytecode::Constant;
use bytecode::OpCode;
use debug::disassemble_chunk;

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

fn main() {
    // let arg: Vec<String> = env::args().collect();
   
    // if arg.len() == 1 {
    //     //run prompt left for now
    // } else {
    //     let file = String::from("src/test.lox");
    //     run_file(&file);
    // }
    // let file = String::from("src/test.lox");
    // run_file(&file);
    let chunk = Chunk {
        code: vec![(OpCode::Constant(0), 123), (OpCode::Return,123)],
        constant: vec![Constant::Number(1.2)]
    };
    disassemble_chunk(&chunk, "test chunk")
}

fn run_file(file_path: &String) {

}

fn run_file_treewalk(file_path: &String) {
    let contents = fs::read_to_string(file_path)
        .expect("Error: file doesnt exist");
    
    let scanner = scanner::scan(contents);
    let parser = parser::parse(scanner.tokens);
    let _interpreter = interpret(parser);
}
