#[cfg(test)]
mod tests {
    use crate::tests::run_from_file;

    #[test]
    fn bool_equality() {
        assert_eq!(run_from_file("src/tests/bool/equality.lox"), ["true", "false", "false", "true", "false", "false", "false", "false", "false", "false", "true", "true", "false", "true", "true", "true", "true", "true"]);

    }

    #[test]
    fn bool_not() {
        assert_eq!(run_from_file("src/tests/bool/not.lox"), ["false", "true", "true"]);
    }
}