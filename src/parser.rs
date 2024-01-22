use std::error::Error;

use crate::expr::{Expr, Literal};
use crate::scanner::{Token, TokenType, self, Scanner};
use crate::stmt::Stmt;

struct Parser {
    tokens: Vec<Token>,
    current: usize
}
enum ParseError {
    Default
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut statements: Vec<Stmt> = vec![];
    let mut parser = Parser {
        tokens,
        current: 0
    };
    while !parser.is_at_end() {
        statements.push(parser.statement())
    }
    return statements
}

impl Parser {
    fn statement(&mut self) -> Stmt {
        if self.token_match(vec![TokenType::PRINT]) {
            return self.print_statement();
        } else {
            return self.expression_statement();
        }
    }

    fn print_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        return Stmt::Print(expr);
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.");
        return Stmt::Expression(expr);
    }

    fn expression(&mut self) -> Expr {
        return self.equailty();
    }

    fn equailty(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.token_match(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right =  Box::new(self.comparison());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right);
        }

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.token_match(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
            let operator = self.previous().clone();
            let right = Box::new(self.term());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right)
        }

        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.token_match(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = Box::new(self.factor());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right)
        }

        return expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.token_match(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right)
        }

        return expr
    }

    fn unary(&mut self) -> Expr {
        if self.token_match(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary());
            return Expr::Unary(operator, right);
        }
        return self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.token_match(vec![TokenType::FALSE]) { 
            return Expr::Literal(Literal::False)
        }
        if self.token_match(vec![TokenType::TRUE]) { 
            return Expr::Literal(Literal::True) 
        }
        if self.token_match(vec![TokenType::NIL]) { 
            return Expr::Literal(Literal::Nil) 
        }
        if self.token_match(vec![TokenType::NUMBER, TokenType::STRING]) { 
            let literal = self.previous().clone().literal;
            match literal {
                Some(literal_type) => match literal_type {
                    scanner::Literal::Str(str) => return Expr::Literal(Literal::String(str)),
                    scanner::Literal::Number(num) => return Expr::Literal(Literal::Number(num)) 
                }
                None => println!("Literal error")
            }
        }
        if self.token_match(vec![TokenType::LEFT_PAREN]) {
            let expr = Box::new(self.expression()); 
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Expr::Grouping(expr); 
        }
        //TODO: make this something betterS
        return Expr::Literal(Literal::Nil)

    }

    fn token_match(& mut self, token_types: Vec<TokenType>) -> bool {
        for token in token_types.iter() {
            if self.check(*token) {
                self.advance();
                return true
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false
        } else {
            return self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end()  {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        //TODO: return this to having EOF with full EOF token
        return self.peek().token_type == TokenType::EOF;
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current -1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance())
        }

        // if let Some(next_token) = self.peek() {
        //     self.error_message(next_token, message)
        // } 
        Err(ParseError::Default)
    }

    fn error_message(self, token: &Token, message: &str) {
        if token.token_type == TokenType::EOF {
            println!("{} at end {}", token.line, message)
        } else {
            println!("{} at '{}' {}", token.line, token.lexeme, message);
        }
    }

}