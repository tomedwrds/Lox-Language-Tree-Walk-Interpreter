#[cfg(test)]
mod tests {
	use crate::tests::run_from_file;

	#[test]
	fn nil_literal() {
		assert_eq!(run_from_file("src/tests/nil/literal.lox"), ["nil"]);
	}

}