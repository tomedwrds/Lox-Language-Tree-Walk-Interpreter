#[cfg(test)]
mod tests {
    use crate::tests::run_from_file;

    #[test]
    fn comments_line_at_eof() {
        assert_eq!(run_from_file("src/tests/comments/line_at_eof.lox"), ["ok"]);

    }

    #[test]
    fn comments_only_line_comment_and_line() {
        assert_eq!(run_from_file("src/tests/comments/only_line_comment_and_line.lox"), Vec::<String>::new());

    }

    #[test]
    fn comments_only_line_comment() {
        assert_eq!(run_from_file("src/tests/comments/only_line_comment.lox"), Vec::<String>::new());

    }

    #[test]
    fn comments_unicode() {
        assert_eq!(run_from_file("src/tests/comments/unicode.lox"), ["ok"]);

    }
}