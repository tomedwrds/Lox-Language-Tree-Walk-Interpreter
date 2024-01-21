#[cfg(test)]
mod tests {
    use crate::{scanner::scan, parser, interpreter::{interpret, Value}};

    //Tests basic addition
    #[test]
    fn expr_test_1() {
        let scanner = scan(String::from("4 + 2 + 1"));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::Number(7.0));
    }

    //Tests order of operations
    #[test]
    fn expr_test_2() {
        let scanner = scan(String::from("3*6 - 7 / 2"));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::Number(14.5));
    }

    //Tests order of operations
    #[test]
    fn expr_test_3() {
        let scanner = scan(String::from("3*6 - 7 / 2 / 2"));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::Number(16.25));
    }

    //Tests unary expression
    #[test]
    fn expr_test_4() {
        let scanner = scan(String::from("(6*(-(-4)))"));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::Number(24.0));
    }

    //Testing basic boolean evaluation
    #[test]
    fn expr_test_5() {
        let scanner = scan(String::from("true==true"));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::Bool(true));
    }

    //Testing basic boolean evaluation
    #[test]
    fn expr_test_6() {
        let scanner = scan(String::from("true!=false"));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::Bool(true));
    }

    //Testing string literal input
    #[test]
    fn expr_test_7() {
        let scanner = scan(String::from(r#""test123""#));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::String("test123".to_string()));
    }

    //Testing string concationation
    #[test]
    fn expr_test_8() {
        let scanner = scan(String::from(r#""hello" + "world""#));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::String("helloworld".to_string()));
    }

    //Testing nil
    #[test]
    fn expr_test_9() {
        let scanner = scan(String::from("nil"));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::Nil);
    }

    //Testing true
    #[test]
    fn expr_test_10() {
        let scanner = scan(String::from("true"));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::Bool(true));
    }

    //Testing false
    #[test]
    fn expr_test_11() {
        let scanner = scan(String::from("false"));
        let parser = parser::parse(scanner.tokens);
        let interpreter = interpret(parser).unwrap();
        assert_eq!(interpreter, Value::Bool(false));
    }
    
}