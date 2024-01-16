use std::collections::HashMap;
use std::fmt;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
  // Single-character tokens.
  LEFT_PAREN,
  RIGHT_PAREN,
  LEFT_BRACE,
  RIGHT_BRACE,
  COMMA,
  DOT,
  MINUS,
  PLUS,
  SEMICOLON,
  SLASH, 
  STAR,

  // One or two character tokens.
  BANG, 
  BANG_EQUAL,
  EQUAL, 
  EQUAL_EQUAL,
  GREATER, 
  GREATER_EQUAL,
  LESS, 
  LESS_EQUAL,

  // Literals.
  IDENTIFIER, 
  STRING, 
  NUMBER,

  // Keywords.
  AND, 
  CLASS, 
  ELSE, 
  FALSE, 
  FUN, 
  FOR, 
  IF, 
  NIL, 
  OR,
  PRINT, 
  RETURN, 
  SUPER, 
  THIS, 
  TRUE, 
  VAR, 
  WHILE,

  EOF
}
#[derive(Debug, Clone)]
pub enum Literal {
  Str(String),
  Number(f64)
}

#[derive(Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Option<Literal>,
  pub line: usize
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
          f,
          "Token {{ ty: {:?}, lexeme: \"{}\", literal: {:?}, line: {:?}}}\n",
          self.token_type,
          self.lexeme,
          self.literal,
          self.line,
      )
  }
}


pub struct Scanner {
  pub source: String,
  pub tokens: Vec<Token>,
  pub start: usize,
  pub current: usize,
  pub line: usize,
  pub keywords: HashMap<String, TokenType>
}

pub fn scan(input: String) -> Scanner {
  let mut scanner: Scanner = Default::default();

  scanner.scan_tokens(input);

  return scanner;
}

impl Default for Scanner {
  fn default() -> Scanner {
      Scanner {
        source: String::new(),
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
      }
  }
}

impl Scanner {

  pub fn display_tokens(self) {
    for t in self.tokens.iter() {
      print!("{}",t);
    }
  }

  fn scan_tokens(&mut self, input: String) {
    self.source = input;
    while !self.scan_finished() {
      self.start = self.current;
      self.scan_token();
    }
  }
  fn scan_token (&mut self) {
    let _char = self.advance();
    match _char {
      '(' => self.add_token(TokenType::LEFT_PAREN),
      ')' => self.add_token(TokenType::RIGHT_PAREN),
      '{' => self.add_token(TokenType::LEFT_BRACE),
      '}' => self.add_token(TokenType::RIGHT_BRACE),
      ',' => self.add_token(TokenType::COMMA),
      '.' => self.add_token(TokenType::DOT),
      '-' => self.add_token(TokenType::MINUS),
      '+' => self.add_token(TokenType::PLUS),
      ';' => self.add_token(TokenType::SEMICOLON),
      '*' => self.add_token(TokenType::STAR),
      '!' => {
        let matches_eq = self._match('=');
        self.add_token(if matches_eq {
          TokenType::BANG_EQUAL
        } else {
          TokenType::BANG
        })},  
      '=' => {
        let matches_eq = self._match('=');
        self.add_token(if matches_eq {
          TokenType::EQUAL_EQUAL
        } else {
          TokenType::EQUAL
        })}, 
      '>' => {
        let matches_eq = self._match('=');
        self.add_token(if matches_eq {
          TokenType::GREATER_EQUAL
        } else {
          TokenType::GREATER
        })}, 
      '<' => {
        let matches_eq = self._match('=');
        self.add_token(if matches_eq {
          TokenType::LESS_EQUAL
        } else {
          TokenType::LESS
        })}, 
      '/' => {
        let matches_eq = self._match('=');
        if matches_eq {
          //Read comment
          while self.peek() != '\n' && !self.scan_finished() {
            self.advance();
          }
        } else {
          self.add_token(TokenType::SLASH)
        }},
      ' ' => (),
      '\r' => (),
      '\t' => (),
      '\n' => {self.line += 1;},
      '"' => self.string(),
      
      _ => {
        if _char.is_digit(10) {
          self.number();
        } else if _char.is_alphabetic() {
          self.identifier();
        } else {
          print!("Invalid token");
        }
      }
    }
  }

  fn identifier(&mut self) {
    while self.peek().is_alphanumeric() {
      self.advance();
    }
    let _text = &self.source[self.start..self.current];
    
    let token_type = match self.keywords.get(_text) {
      Some(kw_token_type) => *kw_token_type,
      None => TokenType::IDENTIFIER,
    };
    self.add_token(token_type);
    
    
  }
  

  fn number(&mut self) {
    while self.peek().is_digit(10) {
      self.advance();
    }
    //Check for decimal point
    if self.peek() == '.' && self.peek_next().is_digit(10) {
      self.advance();
      while self.peek().is_digit(10) {
        self.advance();
      }
    }
    let number: f64 = self.source[self.start..self.current].parse().unwrap();
    self.add_token_literal(TokenType::NUMBER, Some(Literal::Number(number)));
  }
  fn string(&mut self) {
    while self.peek() != '"'  && !self.scan_finished() {
      if self.peek() == '\n' {
        self.line += 1;
      }
      self.advance();
    }
    self.advance();

    //trim quote
    let value = &self.source[self.start+1..self.current-1];
    self.add_token_literal(TokenType::STRING, Some(Literal::Str(String::from(value))));

  }

  fn _match(&mut self, expected: char) -> bool {
    if self.scan_finished() {
      return false
    }
    if self.source.chars().nth(self.current) != Some(expected) {
      return false;
    }
    self.current += 1;
    return true;
  }

  fn add_token(&mut self, token_type: TokenType) {
    self.add_token_literal(token_type, None)
  }

  fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
    let text = &self.source[self.start..self.current];
    self.tokens.push(Token {
      token_type: token_type,
      lexeme: String::from(text),
      literal: literal,
      line: self.line 
    })
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    return self.source.chars().nth(self.current-1).unwrap();
  }

  fn peek(&self) -> char {
    if self.scan_finished() {
      return '\0'
    }
    return self.source.chars().nth(self.current).unwrap();
  }

  fn peek_next(&self) -> char {
    if self.current + 1 >= self.source.len() {
      return '\0'
    }
    return self.source.chars().nth(self.current+1).unwrap();
  }


  fn scan_finished(&self) -> bool {
    return self.current >= self.source.len();
  }
}