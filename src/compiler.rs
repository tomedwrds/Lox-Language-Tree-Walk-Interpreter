use crate::scanner::{scan_bytecode, TokenType};

pub fn compile(src: String) {
    let mut scanner = scan_bytecode(src);
    let mut line = 0;

    while true {
        if let Some(token) = scanner.scan_token() {
            if token.line != line {
                print!("{:<4}",token.line);
                line = token.line;
            } else {
                print!("|   ");
            }
            println!("type: {:?}, lexeme: {}, literal: {:?}", token.token_type, token.lexeme, token.literal);
            if token.token_type == TokenType::EOF {
                break;
            }
        }

    }
}