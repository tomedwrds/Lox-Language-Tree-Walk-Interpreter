#[cfg(test)]
mod tests {

    use crate::scanner::{scan, Token, TokenType};
    #[test]
    fn test_scan_token_left_paren() {
        let scanner = scan(String::from("("));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::LEFT_PAREN, lexeme: String::from("("), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_right_paren() {
        let scanner = scan(String::from(")"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::RIGHT_PAREN, lexeme: String::from(")"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_left_brace() {
        let scanner = scan(String::from("{"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::LEFT_BRACE, lexeme: String::from("{"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_right_brace() {
        let scanner = scan(String::from("}"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::RIGHT_BRACE, lexeme: String::from("}"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_greater() {
        let scanner = scan(String::from(">"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::GREATER, lexeme: String::from(">"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_greater_equal() {
        let scanner = scan(String::from(">="));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::GREATER_EQUAL, lexeme: String::from(">="), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_less() {
        let scanner = scan(String::from("<"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::LESS, lexeme: String::from("<"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_less_equal() {
        let scanner = scan(String::from("<="));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::LESS_EQUAL, lexeme: String::from("<="), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_bang() {
        let scanner = scan(String::from("!"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::BANG, lexeme: String::from("!"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_bang_equal() {
        let scanner = scan(String::from("!="));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::BANG_EQUAL, lexeme: String::from("!="), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_equal() {
        let scanner = scan(String::from("="));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::EQUAL, lexeme: String::from("="), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_equal_equal() {
        let scanner = scan(String::from("=="));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::EQUAL_EQUAL, lexeme: String::from("=="), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_comma() {
        let scanner = scan(String::from(","));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::COMMA, lexeme: String::from(","), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_dot() {
        let scanner = scan(String::from("."));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::DOT, lexeme: String::from("."), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_minus() {
        let scanner = scan(String::from("-"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::MINUS, lexeme: String::from("-"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_plus() {
        let scanner = scan(String::from("+"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::PLUS, lexeme: String::from("+"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_semicolon() {
        let scanner = scan(String::from(";"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::SEMICOLON, lexeme: String::from(";"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_slash() {
        let scanner = scan(String::from("/"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::SLASH, lexeme: String::from("/"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_star() {
        let scanner = scan(String::from("*"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::STAR, lexeme: String::from("*"), literal: None, line: 1}]);
    }

    //Keywords
    #[test]
    fn test_scan_token_and() {
        let scanner = scan(String::from("and"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::AND, lexeme: String::from("and"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_class() {
        let scanner = scan(String::from("class"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::CLASS, lexeme: String::from("class"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_else() {
        let scanner = scan(String::from("else"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::ELSE, lexeme: String::from("else"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_false() {
        let scanner = scan(String::from("false"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::FALSE, lexeme: String::from("false"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_fun() {
        let scanner = scan(String::from("fun"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::FUN, lexeme: String::from("fun"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_for() {
        let scanner = scan(String::from("for"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::FOR, lexeme: String::from("for"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_if() {
        let scanner = scan(String::from("if"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::IF, lexeme: String::from("if"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_nil() {
        let scanner = scan(String::from("nil"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::NIL, lexeme: String::from("nil"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_or() {
        let scanner = scan(String::from("or"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::OR, lexeme: String::from("or"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_print() {
        let scanner = scan(String::from("print"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::PRINT, lexeme: String::from("print"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_return() {
        let scanner = scan(String::from("return"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::RETURN, lexeme: String::from("return"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_super() {
        let scanner = scan(String::from("super"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::SUPER, lexeme: String::from("super"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_this() {
        let scanner = scan(String::from("this"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::THIS, lexeme: String::from("this"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_true() {
        let scanner = scan(String::from("true"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::TRUE, lexeme: String::from("true"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_var() {
        let scanner = scan(String::from("var"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::VAR, lexeme: String::from("var"), literal: None, line: 1}]);
    }
    #[test]
    fn test_scan_token_while() {
        let scanner = scan(String::from("while"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::WHILE, lexeme: String::from("while"), literal: None, line: 1}]);
    }

    #[test]
    fn test_scan_token_comment() {
        let scanner = scan(String::from("//this is a test comment"));
        assert_eq!(scanner.tokens, vec![]);
    }

    #[test]
    fn test_scan_token_comment_next_line() {
        let scanner = scan(String::from("//this is a test comment\nwhile"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::WHILE, lexeme: String::from("while"), literal: None, line: 2}]);
    }
    #[test]
    fn test_scan_token_invalid() {
        let scanner = scan(String::from("^"));
        assert_eq!(scanner.tokens, vec![]);
    }

    #[test]
    fn test_scan_token_literal_string() {
        let scanner = scan(String::from(r#""test""#));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::STRING, lexeme: String::from("\"test\""), literal: Some(crate::scanner::Literal::Str(String::from("test"))), line: 1}]);
    }

    #[test]
    fn test_scan_token_literal_number() {
        let scanner = scan(String::from("124215"));
        assert_eq!(scanner.tokens, vec![Token{token_type: TokenType::NUMBER, lexeme: String::from("124215"), literal: Some(crate::scanner::Literal::Number(124215.0)), line: 1}]);
    }
    
}