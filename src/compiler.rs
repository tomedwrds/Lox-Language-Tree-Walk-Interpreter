
use crate::{bytecode::{Chunk, OpCode, Value}, debug::disassemble_chunk, scanner::{scan, Literal, Scanner, Token, TokenType}};

pub fn compile(src: String) -> Option<Chunk> {
    let mut compiler = compiler_initalize(src);
    compiler.advance();
   
    while !compiler.token_match(TokenType::EOF) {
        compiler.declaration();
    }
    
    compiler.end_compiler();
    return Some(compiler.chunk);
}

struct Compiler {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    chunk: Chunk,
    scanner: Scanner,
}

fn compiler_initalize(src: String) -> Compiler {
    Compiler {
        current: Token { token_type: TokenType::NIL, lexeme: format!(""), literal: None, line: 0 },
        previous: Token { token_type: TokenType::NIL, lexeme: format!(""), literal: None, line: 0 },
        had_error: false,
        panic_mode: false,
        chunk: Chunk::default(),
        scanner: scan(src),
    }
}

impl Compiler {
    fn advance(&mut self) {
        self.previous = self.current.clone();

        self.current = self.scanner.scan_token();
        while self.current.token_type == TokenType::TOKEN_ERROR {
            self.parse_error_token(self.current.clone());
            self.current = self.scanner.scan_token();
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.parse_error(self.current.clone(),Some(message));
    }

    fn emit_byte(&mut self, op_code: OpCode) {
        self.chunk_write(op_code, self.previous.line);
    }

    fn emit_bytes(&mut self, op_1: OpCode, op_2: OpCode) {
        self.emit_byte(op_1);
        self.emit_byte(op_2);
    }

    fn chunk_write(&mut self, op_code: OpCode, line: usize) {
        self.chunk.code.push((op_code, line));
    }

    fn constant_write(&mut self, value: Value) -> usize {
        self.chunk.constant.push(value);
        return self.chunk.constant.len() - 1;
    }

    fn end_compiler(&mut self) {
        self.emit_byte(OpCode::Return)
    }

    fn expression(&mut self) {
        self.parse_precedence(PRECEDENCE.assignment)
    }

    fn declaration(&mut self) {
        if self.token_match(TokenType::VAR) {
            self.declaration_var();
        } else {
            self.statement();
        }
        if self.panic_mode {
            self.synchronize();
        }
    }

    fn declaration_var(&mut self) {
        let global = self.parse_variable(format!("Expect variable name."));

        if self.token_match(TokenType::EQUAL) {
            self.expression();
        } else {
            self.emit_constant(Value::Nil);
        }

        self.consume(TokenType::SEMICOLON, format!("Expect ';' after variable dec"));
        self.define_variable(global);
    }

    fn parse_variable(&mut self, error_message: String) -> usize {
        self.consume(TokenType::IDENTIFIER, error_message);
        let identifier_constant = self.identifier_constant(&self.previous.clone());
        self.chunk.constant.push(Value::String(identifier_constant));
        return self.chunk.constant.len() - 1;

    }

    fn identifier_constant(&mut self, token: &Token) -> String {
        return token.lexeme.clone();
    }

    fn define_variable(&mut self, global: usize) {
        self.emit_byte(OpCode::DefineGlobal(global))
    }

    fn statement(&mut self) {
       if self.token_match(TokenType::PRINT) {
        self.statement_print()
       } else {
        self.statement_expression()
       }
    }

    fn statement_print(&mut self) {
        self.expression();
        self.consume(TokenType::SEMICOLON, format!("Expect ';' after value."));
        self.emit_byte(OpCode::Print)
    }

    fn statement_expression(&mut self) {
        self.expression();
        self.consume(TokenType::SEMICOLON, format!("Expect ';' after expression."));
        self.emit_byte(OpCode::Pop);
    }

    fn token_match(&mut self, token_type: TokenType) -> bool {
        if self.current.token_type == token_type {
            self.advance();
            return true
        }
        return false
    }


    fn parse_precedence(&mut self, precedence: u8) {
        self.advance();
        let prefix_rule = get_rules(self.previous.token_type).prefix;
        if let Some(prefix_func) = prefix_rule {
            
            let can_assign = precedence <= PRECEDENCE.assignment;
            prefix_func(self,can_assign);

            while precedence <= get_rules(self.current.token_type).precedence {
                self.advance();
                let infix_rule =get_rules(self.previous.token_type).infix;
                if let Some(infix_func) = infix_rule {
                    infix_func(self, can_assign);
                }

                if can_assign && self.token_match(TokenType::EQUAL) {
                    self.parse_error(self.current.clone(), Some(format!("Invalid assignment target.")));
                  }
            }
        } else {
            self.parse_error(self.previous.clone(), Some(format!("Expect expression.")))
        }
    }

    fn emit_constant(&mut self, value: Value) {
        self.chunk.constant.push(value);
        self.emit_byte(OpCode::Constant(self.chunk.constant.len() - 1));
    }

    fn parse_error_token(&mut self, token: Token) {
        if let Some(error_literal) = &token.literal {
            if let Literal::Str(error_message) = error_literal {
                self.parse_error(token.clone(), Some(error_message.to_string()));
                return;
            }
        }
        self.parse_error(token, None);
    }


    fn parse_error(&mut self, token: Token, error_message: Option<String>) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        println!("[line {}] Error at token {:?}", token.line, token.token_type);
        if let Some(error_message_set) = error_message {
            println!("Error Message: {:?}", error_message_set)
        } 
        self.had_error = true;
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        while self.current.token_type != TokenType::EOF {
            if self.previous.token_type == TokenType::SEMICOLON {
                return;
            }

            match self.current.token_type {
                TokenType::CLASS => return,
                TokenType::FUN => return,
                TokenType::VAR => return,
                TokenType::FOR => return,
                TokenType::IF => return,
                TokenType::WHILE => return,
                TokenType::PRINT => return,
                TokenType::RETURN => return,
                _ => ()
            }
            self.advance();
        }
     }
}

const PRECEDENCE: Precedence = Precedence {
    none: 0,
    assignment: 1,
    or: 2,
    and: 3,
    equality: 4,
    comparison: 5,
    term: 6,
    factor: 7,
    unary: 8,
    call: 9,
    primary: 10
};

fn get_rules(token: TokenType) -> Rule {
    match token {
        TokenType::LEFT_PAREN => Rule{prefix: Some(grouping), infix: None, precedence: PRECEDENCE.none },
        TokenType::MINUS => Rule{prefix: Some(unary), infix: Some(binary), precedence: PRECEDENCE.term },
        TokenType::PLUS => Rule{prefix: None, infix: Some(binary), precedence: PRECEDENCE.term },
        TokenType::SLASH => Rule{prefix: None, infix: Some(binary), precedence: PRECEDENCE.factor },
        TokenType::STAR => Rule{prefix: None, infix: Some(binary), precedence: PRECEDENCE.factor },
        TokenType::BANG => Rule{prefix: Some(unary), infix: None, precedence: PRECEDENCE.none},
        TokenType::STRING => Rule{prefix: Some(value_literal), infix: None, precedence: PRECEDENCE.none},
        TokenType::IDENTIFIER => Rule { prefix: Some(variable), infix: None, precedence: PRECEDENCE.none },
        TokenType::NUMBER => Rule{prefix: Some(value_literal), infix: None, precedence: PRECEDENCE.none },
        TokenType::FALSE => Rule{prefix: Some(literal), infix: None, precedence: PRECEDENCE.none},
        TokenType::TRUE => Rule{prefix: Some(literal), infix: None, precedence: PRECEDENCE.none},
        TokenType::NIL => Rule{prefix: Some(literal), infix: None, precedence: PRECEDENCE.none},
        TokenType::BANG_EQUAL => Rule{prefix: None, infix: Some(binary), precedence: PRECEDENCE.equality},
        TokenType::EQUAL_EQUAL => Rule{prefix: None, infix: Some(binary), precedence: PRECEDENCE.equality},
        TokenType::GREATER => Rule{prefix: None, infix: Some(binary), precedence: PRECEDENCE.comparison},
        TokenType::GREATER_EQUAL => Rule{prefix: None, infix: Some(binary), precedence: PRECEDENCE.comparison},
        TokenType::LESS => Rule{prefix: None, infix: Some(binary), precedence: PRECEDENCE.comparison},
        TokenType::LESS_EQUAL => Rule{prefix: None, infix: Some(binary), precedence: PRECEDENCE.comparison},
        _ => Rule{prefix: None, infix: None, precedence: PRECEDENCE.none}
    }
}

struct Rule {
    prefix: Option<fn(&mut Compiler, can_assign: bool) -> ()>,
    infix: Option<fn(&mut Compiler, can_assign: bool) -> ()>,
    precedence: u8
}

struct Precedence {
    none: u8,
    assignment: u8,
    or: u8,
    and: u8,
    equality: u8,
    comparison: u8,
    term: u8,
    factor: u8,
    unary: u8,
    call: u8,
    primary: u8
}


fn grouping(compiler: &mut Compiler, can_assign: bool) {
    compiler.expression();
    compiler.consume(TokenType::RIGHT_PAREN, format!("Excpect ')' after expression."));
}

fn value_literal(compiler: &mut Compiler, can_assign: bool) {
    //TODO: better error handling here (we can assume for now value set)
    let value = &compiler.previous.literal;
    if let Some(value_set) = value {
        match value_set {
            Literal::Number(num) => {
                compiler.emit_constant(Value::Number(*num)); 
            },
            Literal::Str(str) => {
                compiler.emit_constant(Value::String(format!("{}",str)));
            }
        }
    }
   
} 


fn unary(compiler: &mut Compiler, can_assign: bool) {
    let operator = compiler.previous.token_type;

    //Compile the operand
    compiler.parse_precedence(PRECEDENCE.unary);

    //Emit the operator instruction
    match operator {
        TokenType::MINUS => compiler.emit_byte(OpCode::Negate),
        TokenType::BANG => compiler.emit_byte(OpCode::Not),
        _ => ()
    }
}

fn binary(compiler: &mut Compiler, can_assign: bool) {
    let operator_type = compiler.previous.token_type;
    let rule = get_rules(operator_type);
    compiler.parse_precedence(rule.precedence + 1);

    match operator_type {
        TokenType::PLUS => compiler.emit_byte(OpCode::Add),
        TokenType::MINUS => compiler.emit_byte(OpCode::Subtract),
        TokenType::STAR => compiler.emit_byte(OpCode::Multiply),
        TokenType::SLASH => compiler.emit_byte(OpCode::Divide),
        TokenType::BANG_EQUAL => compiler.emit_bytes(OpCode::Equal, OpCode::Not),
        TokenType::EQUAL_EQUAL => compiler.emit_byte(OpCode::Equal),
        TokenType::LESS => compiler.emit_byte(OpCode::Less),
        TokenType::GREATER_EQUAL => compiler.emit_bytes(OpCode::Less, OpCode::Not),
        TokenType::GREATER => compiler.emit_byte(OpCode::Greater),
        TokenType::LESS_EQUAL => compiler.emit_bytes(OpCode::Greater, OpCode::Not),
        _ => ()
    }
}

fn literal(compiler: &mut Compiler, can_assign: bool) {
    match compiler.previous.token_type {
        TokenType::FALSE => compiler.emit_constant(Value::Bool(false)),
        TokenType::NIL => compiler.emit_constant(Value::Nil),
        TokenType::TRUE => compiler.emit_constant(Value::Bool(true)),
        _ => ()
    }
}

fn variable(compiler: &mut Compiler, can_assign: bool) {
    named_variable(compiler, &compiler.previous.clone(), can_assign);
}

fn named_variable(compiler: &mut Compiler, token: &Token, can_assign: bool) {
    let arg = compiler.identifier_constant(&token);

    if compiler.token_match(TokenType::EQUAL) && can_assign {
        compiler.expression();
        compiler.emit_byte(OpCode::SetGlobal(arg))
    } else {
        compiler.emit_byte(OpCode::GetGlobal(arg));
    }
}