#[cfg(test)]
mod tests {
    use crate::tests::run_from_file;

    #[test]
    fn assignment_associativity() {
        assert_eq!(run_from_file("src/tests/assignment/associativity.lox"), ["c", "c", "c"]);

    }

    #[test]
    fn assignment_global() {
        assert_eq!(run_from_file("src/tests/assignment/global.lox"), ["before", "after", "arg", "arg"]);
    }

    #[test]
    fn assignment_grouping() {
        assert_eq!(run_from_file("src/tests/assignment/grouping.lox"), ["[Line 2] Error at token EQUAL", "Error Message: \"Invalid assignment target.\""]);
    }

    #[test]
    fn assignment_infix_operator() {
        assert_eq!(run_from_file("src/tests/assignment/grouping.lox"), ["[Line 2] Error at token EQUAL", "Error Message: \"Invalid assignment target.\""]);
    }

    #[test]
    fn assignment_local() {
        assert_eq!(run_from_file("src/tests/assignment/local.lox"), ["before", "after", "arg", "arg"]);
    }

    #[test]
    fn assignment_syntax() {
        assert_eq!(run_from_file("src/tests/assignment/syntax.lox"), ["var", "var"]);
    }
}