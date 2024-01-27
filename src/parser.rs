use crate::expr::{Expr, Literal};
use crate::scanner::{Token, TokenType, self, Scanner};
use crate::stmt::Stmt;

struct Parser {
    tokens: Vec<Token>,
    current: usize
}
enum ParseError {
    Default,
    Assignment
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut statements: Vec<Stmt> = vec![];
    let mut parser = Parser {
        tokens,
        current: 0
    };
    while !parser.is_at_end() {
        if let Ok(dec) = parser.declaration() {
            statements.push(dec);
        } else {
            parser.synchronize();
        }
    }
    return statements
}

impl Parser {
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.token_match(vec![TokenType::VAR]) {
            return self.var_declaration();
        }
        return self.statement();
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let token = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?.clone();

        let mut intitalizer = Expr::Literal(Literal::Nil);
        if self.token_match(vec![TokenType::EQUAL]) {
            intitalizer = self.expression()?;
        }

        self.consume(TokenType::SEMICOLON, "Expect ';' after variable declaration.");
        return Ok(Stmt::Var(token, intitalizer));

    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.token_match(vec![TokenType::PRINT]) {
            return self.print_statement();
        } else if self.token_match(vec![TokenType::LEFT_BRACE]) {
            return Ok(Stmt::Block(self.block_statement()?)); 
        } else if self.token_match(vec![TokenType::IF]) {
            return self.if_statement(); 
        } else {
            return self.expression_statement();
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' fater 'if'.");
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' fater if condition.");

        let then_branch = self.statement()?;
        let mut else_branch = None;
        if self.token_match(vec![TokenType::ELSE]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        return Ok(Stmt::If(condition, Box::new(then_branch), else_branch))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after print.");
        return Ok(Stmt::Print(expr));
    }

    fn block_statement(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RIGHT_BRACE, "Except '}' after block.");
        return Ok(statements);
    }


    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        return Ok(Stmt::Expression(expr));
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        return self.assingment();
    }

    fn assingment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.equailty()?;
        if self.token_match(vec![TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assingment()?;

            match expr {
                Expr::Variable(token) => return Ok(Expr::Assign(token, Box::new(value))),
                _ => {
                    print!("Invalid assignment target");
                }
            }
        }
        return Ok(expr);
    }

    fn equailty(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparison()?;

        while self.token_match(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right =  Box::new(self.comparison()?);
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right);
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.term()?;

        while self.token_match(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
            let operator = self.previous().clone();
            let right = Box::new(self.term()?);
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right)
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.factor()?;

        while self.token_match(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = Box::new(self.factor()?);
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right)
        }

        return Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.unary()?;

        while self.token_match(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary(left, operator, right)
        }

        return Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.token_match(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            return Ok(Expr::Unary(operator, right));
        }
        return self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.token_match(vec![TokenType::FALSE]) { 
            return Ok(Expr::Literal(Literal::False))
        }
        if self.token_match(vec![TokenType::TRUE]) { 
            return Ok(Expr::Literal(Literal::True))
        }
        if self.token_match(vec![TokenType::NIL]) { 
            return Ok(Expr::Literal(Literal::Nil))
        }
        if self.token_match(vec![TokenType::NUMBER, TokenType::STRING]) { 
            let literal = self.previous().clone().literal;
            match literal {
                Some(literal_type) => match literal_type {
                    scanner::Literal::Str(str) => return Ok(Expr::Literal(Literal::String(str))),
                    scanner::Literal::Number(num) => return Ok(Expr::Literal(Literal::Number(num))) 
                }
                None => println!("Literal error")
            }
        }
        if self.token_match(vec![TokenType::IDENTIFIER]) {
            return Ok(Expr::Variable(self.previous().clone()))
        }
        if self.token_match(vec![TokenType::LEFT_PAREN]) {
            let expr = Box::new(self.expression()?); 
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(expr)); 
        }
        //TODO: make this something betterS
        return Ok(Expr::Literal(Literal::Nil))

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
        let problem_token = self.peek();
        if problem_token.token_type == TokenType::EOF {
            println!("Line {} at end {}", problem_token.line, message)
        } else {
            println!("Line {} at '{}' {}", problem_token.line, problem_token.lexeme, message);
        }
        Err(ParseError::Default)
    }


    fn synchronize(&mut self) {
        self.advance();
    
        while !self.is_at_end() {
          if self.previous().token_type == TokenType::SEMICOLON {
            return;
          }
          match self.peek().token_type {
            TokenType::CLASS => return,
            TokenType::FUN => return,
            TokenType::VAR => return,
            TokenType::FOR => return,
            TokenType::IF => return,
            TokenType::WHILE => return,
            TokenType::PRINT => return,
            TokenType::RETURN => return,
            _ => {self.advance(); return;}
          }
        }
      }

}