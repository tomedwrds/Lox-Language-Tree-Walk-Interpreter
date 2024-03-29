
use std::fmt::format;

use crate::{bytecode::{Chunk, OpCode, Value}, debug::disassemble_chunk, scanner::{scan, Literal, Scanner, Token, TokenType}};

pub fn compile(src: String) -> CompilerOutput {
    let mut compiler = compiler_initalize(src, FunctionType::Script);
    compiler.advance();
   
    while !compiler.token_match(TokenType::EOF) {
        compiler.declaration();
    }
    
    compiler.end_compiler();
    
    if compiler.error_message.is_empty() {
        return CompilerOutput::Success(compiler.function)
    }  
    return CompilerOutput::Error(compiler.error_message)
}

pub struct Function {
    arity: i32,
    chunk: Chunk,
    pub name: Option<String>,
}

enum FunctionType {
    Function,
    Script
}

fn new_function() -> Function {
    Function {
        arity: 0,
        name: None,
        chunk: Chunk::default()
    }
}

pub enum CompilerOutput {
    Success(Function),
    Error(Vec<String>)
}
struct Compiler {
    current: Token,
    previous: Token,
    error_message: Vec<String>,
    panic_mode: bool,
    scanner: Scanner,
    locals: Vec<Local>,
    scope_depth: i32,
    in_loop: bool,
    function: Function,
    function_type: FunctionType
}

#[derive(Clone, PartialEq, Debug)]
pub struct Local {
    pub name: Token,
    pub depth: i32,
    is_const: bool
}


fn compiler_initalize(src: String, func_type: FunctionType) -> Compiler {
    Compiler {
        current: Token { token_type: TokenType::NIL, lexeme: format!(""), literal: None, line: 0 },
        previous: Token { token_type: TokenType::NIL, lexeme: format!(""), literal: None, line: 0 },
        error_message: vec![],
        panic_mode: false,
        function_type: func_type,
        function: new_function(),
        scanner: scan(src),
        scope_depth: 0,
        locals: Vec::new(),
        in_loop: false,
    }
}

impl Compiler {
    fn current_chunk(&mut self) -> &Chunk {
       return &self.function.chunk;
    }

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
        self.current_chunk().code.push((op_code, line));
    }

    fn constant_write(&mut self, value: Value) -> usize {
        self.current_chunk().constant.push(value);
        return self.current_chunk().constant.len() - 1;
    }

    fn end_compiler(&mut self) {
        self.emit_byte(OpCode::Return)
    }

    fn expression(&mut self) {
        self.parse_precedence(PRECEDENCE.assignment)
    }

    fn declaration(&mut self) {
        if self.token_match(TokenType::VAR) {
            self.declaration_var(false);

        } else if self.token_match(TokenType::CONST) {
            self.declaration_var(true);

        } else {
            self.statement(self.in_loop);
        }
        if self.panic_mode {
            self.synchronize();
        }
    }

    fn declaration_var(&mut self, is_const: bool) {
        let global = self.parse_variable(format!("Expect variable name."), is_const);

        if self.token_match(TokenType::EQUAL) {
            self.expression();
        } else {
            self.emit_constant(Value::Nil);
        }

        self.consume(TokenType::SEMICOLON, format!("Expect ';' after variable dec"));
        self.define_variable(global, is_const);
    }

    fn parse_variable(&mut self, error_message: String, is_const: bool) -> usize {
        self.consume(TokenType::IDENTIFIER, error_message);

        self.declare_variable(is_const);
        
        if self.scope_depth > 0 {
            return 0
        }

        let identifier_constant = self.identifier_constant(&self.previous.clone());
        self.current_chunk().constant.push(Value::String(identifier_constant));
        return self.current_chunk().constant.len() - 1;

    }

    fn identifier_constant(&mut self, token: &Token) -> String {
        return token.lexeme.clone();
    }

    fn declare_variable(&mut self, is_const: bool) {
        if self.scope_depth == 0 {
            return;
        }

        let token = self.previous.clone();
        self.add_local(token, is_const);
    }

    fn add_local(&mut self, token: Token, is_const: bool) {
        let local =  Local {name: token.clone(), depth: -1, is_const};
        for existing_locals in self.locals.clone() {
            if existing_locals.name.lexeme == token.lexeme && self.scope_depth == existing_locals.depth {
                self.parse_error(self.previous.clone(), Some(format!("Already a variable with this name in this scope.")));
            }
        }
        self.locals.push(local);
    }

    fn define_variable(&mut self, global: usize, is_const: bool) {
        if self.scope_depth > 0 {
            self.mark_initalized();
            return;
        }
        self.emit_byte(OpCode::DefineGlobal(global, is_const))
    }

    fn mark_initalized(&mut self) {
        let index = self.locals.len() - 1;
        self.locals[index].depth = self.scope_depth;
    }

    fn statement(&mut self, in_loop: bool) {
        if self.token_match(TokenType::PRINT) {
            self.statement_print();
        } else if self.token_match(TokenType::IF) {
            self.statement_if();
        } else if self.token_match(TokenType::SWITCH) {
            self.statement_switch();
        } else if self.token_match(TokenType::WHILE) {
            self.statement_while();
        } else if self.token_match(TokenType::FOR) {
            self.statement_for();
        } else if self.token_match(TokenType::LEFT_BRACE) {
            self.begin_scope(in_loop);
            self.statement_block();
            self.end_scope();
        } else if self.token_match(TokenType::BREAK) {
            if !self.in_loop {
                self.parse_error(self.previous.clone(), Some(format!("Break statements only allowed in 'for' or 'while' loops.")));
            } else {
                self.consume(TokenType::SEMICOLON, format!("Expect ';' after 'break'."));
                self.emit_byte(OpCode::Break);
            }
        } else if self.token_match(TokenType::CONTINUE) {
            if !self.in_loop {
                self.parse_error(self.previous.clone(), Some(format!("Continue statements only allowed in 'for' or 'while' loops.")));
            } else {
                self.consume(TokenType::SEMICOLON, format!("Expect ';' after 'continue'."));
                self.emit_byte(OpCode::Continue);
            }
        } else {
            self.statement_expression()
        }
    }

    fn statement_switch(&mut self) {
        self.consume(TokenType::LEFT_PAREN, format!("Expect '(' after 'switch'."));
        self.expression();
        self.consume(TokenType::RIGHT_PAREN, format!("Expect ')' after switched value."));
        
        self.consume(TokenType::LEFT_BRACE, format!("Expect '{{' at start of switch."));
        
        let mut case_end_jumps: Vec<usize> = vec![];
        while self.token_match(TokenType::CASE) {
            self.expression();
            self.consume(TokenType::COLON, format!("Expect ':' after 'case'."));
            let case_jump = self.emit_jump(OpCode::SwitchJump(0xff));
            
            self.statement(self.in_loop);
            let jump_index = self.emit_jump(OpCode::Jump(0xff));
            case_end_jumps.push(jump_index);

            self.patch_jump(case_jump);
        }

        self.consume(TokenType::DEFAULT, format!("Expect 'default' at end of switch."));
        self.consume(TokenType::COLON, format!("Expect ':' after 'swtich'."));
        self.statement(self.in_loop);

        self.consume(TokenType::RIGHT_BRACE, format!("Expect '}}' at end of switch."));

        for case_end in case_end_jumps {
            self.patch_jump(case_end);
        }
        self.emit_byte(OpCode::Pop);


    }

    fn statement_for(&mut self) {
        self.begin_scope(true);
        self.consume(TokenType::LEFT_PAREN, format!("Expect '(' after 'for'."));


        if self.token_match(TokenType::SEMICOLON) {

        } else if self.token_match(TokenType::VAR) {
            self.declaration_var(false);
        } else {
            self.statement_expression();
        }
        let mut loop_start  = self.current_chunk().code.len();
        let mut exit_jump: Option<usize> = None;
        if !self.token_match(TokenType::SEMICOLON) {
            self.expression();
            self.consume(TokenType::SEMICOLON, format!("Expect ';' after loop condition."));

            //Jump out of loop if the condition is false;
            exit_jump = Some(self.emit_jump(OpCode::JumpIfFalse(0xff)));
            self.emit_byte(OpCode::Pop);
        }

        if !self.token_match(TokenType::RIGHT_PAREN) {
            let body_jump = self.emit_jump(OpCode::Jump(0xff));
            let increment_start = self.current_chunk().code.len();

            self.expression();
            self.emit_byte(OpCode::Pop);
            self.consume(TokenType::RIGHT_PAREN, format!("Expect ')' after for clauses."));

            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);

        }

        self.statement(true);
        self.emit_loop(loop_start);

        if let Some(some_exit_jump) = exit_jump {
            self.patch_jump(some_exit_jump);
            self.emit_byte(OpCode::Pop);
        }
        self.end_scope();
    }

    fn statement_while(&mut self) {
        let loop_start = self.current_chunk().code.len();
        self.consume(TokenType::LEFT_PAREN, format!("Expect '(' after 'while'."));
        self.expression();
        self.consume(TokenType::RIGHT_PAREN, format!("Expect ')' after condition."));

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0xff));
        self.emit_byte(OpCode::Pop);
        self.statement(true);
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump);
        self.emit_byte(OpCode::Pop);
    }

    fn emit_loop(&mut self, loop_start: usize) {
        let chunk_length = self.current_chunk().code.len();
        self.emit_byte(OpCode::Loop(chunk_length - loop_start + 1));
    }

    fn statement_if(&mut self) {
        self.consume(TokenType::LEFT_PAREN, format!("Expect '(' after 'if'."));
        self.expression();
        self.consume(TokenType::RIGHT_PAREN, format!("Expect ')' after condition."));

        let then_jump = self.emit_jump(OpCode::JumpIfFalse(0xff));
        self.emit_byte(OpCode::Pop);
        self.statement(self.in_loop);

        let else_jump = self.emit_jump(OpCode::Jump(0xff));
        self.patch_jump(then_jump);
        self.emit_byte(OpCode::Pop);

        if self.token_match(TokenType::ELSE) {
            self.statement(self.in_loop);
        }
        self.patch_jump(else_jump);
    }

    fn emit_jump(&mut self, instruction: OpCode) -> usize {
        self.emit_byte(instruction);
        return self.current_chunk().code.len()-1;
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump_size = self.current_chunk().code.len() - 1 -offset;
        let (opcode, line) = &self.current_chunk().code[offset];
        match opcode {
            OpCode::JumpIfFalse(_) => self.current_chunk().code[offset] = (OpCode::JumpIfFalse(jump_size), *line),
            OpCode::Jump(_) => self.current_chunk().code[offset] = (OpCode::Jump(jump_size), *line),
            OpCode::SwitchJump(_) => self.current_chunk().code[offset] = (OpCode::SwitchJump(jump_size), *line),
            _ => panic!("Attempting to patch the jump of non jump opcode")
        }
        
    }

    fn statement_block(&mut self) {
        while !(self.current.token_type == TokenType::RIGHT_BRACE || self.current.token_type == TokenType::EOF) {
            self.declaration();
        }

        self.consume(TokenType::RIGHT_BRACE, format!("Expect '}}' after block."));
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

    fn begin_scope(&mut self, in_loop: bool) {
        self.scope_depth += 1;
        self.in_loop = in_loop;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        let mut local_count = self.locals.len();
        while local_count > 0 && self.locals[local_count - 1].depth > self.scope_depth {
            self.emit_byte(OpCode::Pop);
            self.locals.pop();
            local_count -= 1;
        }
        self.in_loop = false;
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
                
            }
            if can_assign && self.token_match(TokenType::EQUAL) {
                self.parse_error(self.previous.clone(), Some(format!("Invalid assignment target.")));
            }

        } else {
            self.parse_error(self.previous.clone(), Some(format!("Expect expression.")))
        }
    }

    fn emit_constant(&mut self, value: Value) {
        self.current_chunk().constant.push(value);
        let chunk_length = self.current_chunk().constant.len();
        self.emit_byte(OpCode::Constant(chunk_length - 1));
    }

    fn named_variable(&mut self, token: &Token, can_assign: bool) {    
        let get_op: OpCode;
        let set_op: OpCode;

        if let Some(arg) = self.resolve_local(&token) {
            get_op = OpCode::GetLocal(arg);
            set_op = OpCode::SetLocal(arg);
            if self.locals[arg].is_const {
                self.parse_error(self.previous.clone(), Some(format!("Can't reassign constant variable.")));
            }
        } else {
            let name_arg = self.identifier_constant(&token);
            get_op = OpCode::GetGlobal(name_arg.clone());
            set_op = OpCode::SetGlobal(name_arg);
        }
    
    
        if can_assign {
            if self.token_match(TokenType::EQUAL) {
                self.expression();
                self.emit_byte(set_op)
            } else {
                self.emit_byte(get_op);
            }
        } else {
            self.parse_error(self.current.clone(), Some(format!("Invalid assignment target.")));
        }
    }

    fn resolve_local(&mut self, token: &Token) -> Option<usize> {
        let mut local_count = self.locals.len();
        while local_count > 0 {
            if let Some(value) = self.locals.get(local_count - 1) {
                if value.name.lexeme == *token.lexeme {
                    if self.locals[local_count-1].depth == -1 {
                        self.parse_error(self.previous.clone(), Some(format!("Can't read local variable in its own initializer.")));
                    }
                    return Some(local_count-1)
                }
            }
            local_count -= 1;
        }
        return None
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
        self.error_message.push(format!("[Line {}] Error at '{}'", token.line, token.lexeme));
        if let Some(error_message_set) = error_message {
            self.error_message.push(format!("Error Message: {}", error_message_set));
        } 
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
        TokenType::AND => Rule{prefix: None, infix: Some(and_), precedence: PRECEDENCE.and},
        TokenType::OR => Rule{prefix: None, infix: Some(or_), precedence: PRECEDENCE.or},
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
    compiler.named_variable(&compiler.previous.clone(), can_assign);
}

fn and_(compiler: &mut Compiler, can_assign: bool) {
    let end_jump = compiler.emit_jump(OpCode::JumpIfFalse(0xff));
    compiler.emit_byte(OpCode::Pop);
    compiler.parse_precedence(PRECEDENCE.and);
    compiler.patch_jump(end_jump);
}

fn or_(compiler: &mut Compiler, can_assign: bool) {
    let else_jump = compiler.emit_jump(OpCode::JumpIfFalse(0xff));
    let end_jump = compiler.emit_jump(OpCode::Jump(0xff));
    
    compiler.patch_jump(else_jump);
    compiler.emit_byte(OpCode::Pop);

    compiler.parse_precedence(PRECEDENCE.or);
    compiler.patch_jump(end_jump);
}

