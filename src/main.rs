use std::env;
use std::fs;

use crate::interpreter::interpret;
mod scanner;
mod expr;
mod parser;
mod interpreter;
mod stmt;
mod enviroment;
mod lox_callable;
mod lox_instance;


fn main() {
    let arg: Vec<String> = env::args().collect();
   
    if arg.len() == 1 {
        //run prompt left for now
    } else {
        let file = String::from("src/test.lox");
        run_file(&file);
    }
    let file = String::from("src/test.lox");
    run_file(&file);
   
}

fn run_file(file_path: &String) {
    let contents = fs::read_to_string(file_path)
        .expect("Error: file doesnt exist");
    
    let scanner = scanner::scan(contents);
    scanner.display_tokens();
    let parser = parser::parse(scanner.tokens);
    let _interpreter = interpret(parser);
}
