use crate::expr::{Expr, Literal};
use crate::scanner::{Token, TokenType, self, Scanner};

struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    fn expression(&mut self) -> Expr {
        return self.equailty();
    }

    fn equailty(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.token_match(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator: Token = self.previous();
            let right =  Box::new(self.comparison());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right);
        }

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.token_match(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
            let operator: Token = self.previous();
            let right = Box::new(self.term());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right)
        }

        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.token_match(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = self.previous();
            let right = Box::new(self.factor());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right)
        }

        return expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.token_match(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = self.previous();
            let right = Box::new(self.unary());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right)
        }

        return expr
    }

    fn unary(&mut self) -> Expr {
        if self.token_match(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = self.previous();
            let right = Box::new(self.unary());
            return Expr::Unary(operator, right);
        }
        return self.primary()
    }

    fn primary(self) -> Expr {
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
            let literal = self.previous().literal;
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

    fn check(self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false
        } else {
            return self.peek().token_type == token_type
        }
    }

    fn advance(& mut self) -> Token {
        if !self.is_at_end()  {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    fn peek(self) -> Token {
        return self.tokens[self.current];
    }

    fn previous(self) -> Token {
        return self.tokens[self.current -1]
    }

}