use std::{str, collections::HashMap};
use std::fmt;
#[derive(Copy, Clone, Debug)]
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
enum Literal {
  Str(String),
  Number(f64)
}

#[derive(Clone)]
pub struct Token {
  token_type: TokenType,
  lexeme: Vec<u8>,
  literal: Option<Literal>,
  line: usize
}
impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
          f,
          "Token {{ ty: {:?}, lexeme: \"{}\", literal: {:?}, line: {:?}}}",
          self.token_type,
          String::from_utf8(self.lexeme.clone()).unwrap(),
          self.literal,
          self.line,
      )
  }
}

impl Token {
  pub fn print_token(&self) -> &str {
    "Token"
  }
}

pub struct Scanner {
  pub source: Vec<u8>,
  pub tokens: Vec<Token>,
  pub start: usize,
  pub current: usize,
  pub line: usize,
  pub keywords: HashMap<String, TokenType>
}

impl Scanner {
  // pub fn print_file(&self) {
  //   print!("{}",self.source);
  // }

  pub fn scan_tokens(&mut self) {
    
    while !self.scan_finished() {
      self.start = self.current;
      self.scan_token();
    }
    for t in self.tokens.iter() {
      print!("{}",t);
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
    let _text = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();
    
    let token_type = match self.keywords.get(&_text) {
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
    let number: f64 = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap().parse().unwrap();
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
    let value = String::from_utf8(self.source[self.start+1..self.current-1].to_vec()).unwrap();
    self.add_token_literal(TokenType::STRING, Some(Literal::Str(value)));

  }

  fn _match(&mut self, expected: char) -> bool {
    if self.scan_finished() {
      return false
    }
    if self.source[self.current] as char != expected {
      return false;
    }
    self.current += 1;
    return true;
  }

  fn add_token(&mut self, token_type: TokenType) {
    self.add_token_literal(token_type, None)
  }

  fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
    let text = self.source[self.start..self.current].to_vec();
    self.tokens.push(Token {
      token_type: token_type,
      lexeme: text,
      literal: literal,
      line: self.line 
    })
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    return self.source[self.current-1] as char;
  }

  fn peek(&self) -> char {
    if self.scan_finished() {
      return '\0'
    }
    return self.source[self.current] as char;
  }

  fn peek_next(&self) -> char {
    if self.current + 1 >= self.source.len() {
      return '\0'
    }
    return self.source[self.current + 1] as char;
  }


  fn scan_finished(&self) -> bool {
    return self.current >= self.source.len();
  }
}
