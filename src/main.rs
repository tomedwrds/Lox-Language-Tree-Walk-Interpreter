use std::collections::HashMap;
use std::env;
use std::fs;
mod scanner;
use crate::scanner::TokenType;
fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() == 1 {
        //run prompt left for now
       
    } else {
        run_file(&arg[1])
    }
}

fn run_file(file_path: &String) {
    let contents = fs::read_to_string(file_path)
        .expect("Error: file doesnt exist");
    
    let mut scanner = scanner::Scanner {
        source: contents,
        tokens: Vec::new(),
        current: 0,
        start: 0,
        line: 1,
        keywords: HashMap::from([
            (String::from("and"), TokenType::AND),
            (String::from("class"), TokenType::CLASS),
            (String::from("else"), TokenType::ELSE),
            (String::from("false"), TokenType::FALSE),
            (String::from("for"), TokenType::FOR),
            (String::from("fun"), TokenType::FUN),
            (String::from("if"), TokenType::IF),
            (String::from("nil"), TokenType::NIL),
            (String::from("or"), TokenType::OR),
            (String::from("print"), TokenType::PRINT),
            (String::from("return"), TokenType::RETURN),
            (String::from("super"), TokenType::SUPER),
            (String::from("this"), TokenType::THIS),
            (String::from("true"), TokenType::TRUE),
            (String::from("var"), TokenType::VAR),
            (String::from("while"), TokenType::WHILE),
        ])
    };

    scanner.scan_tokens();
}
