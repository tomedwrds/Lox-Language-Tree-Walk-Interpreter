#[cfg(test)]
mod tests {
    use crate::tests::run_from_file;

    #[test]
    fn block_empty() {
        assert_eq!(run_from_file("src/tests/block/empty.lox"), ["ok"]);

    }

    #[test]
    fn block_scope() {
        assert_eq!(run_from_file("src/tests/block/scope.lox"), ["inner", "outer"]);
    }
}