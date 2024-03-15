#[cfg(test)]
mod tests {
	use crate::tests::run_from_file;

	#[test]
	fn number_decimal_point_at_eof() {
	}

	#[test]
	fn number_leading_dot() {
		assert_eq!(run_from_file("src/tests/number/leading_dot.lox"), ["[Line 1] Error at '.'", "Error Message: Expect expression."]);
	}

	#[test]
	fn number_literals() {
		assert_eq!(run_from_file("src/tests/number/literals.lox"), ["123", "987654", "0", "-0", "123.456", "-0.001"]);
	}

	#[test]
	fn number_nan_equality() {
		assert_eq!(run_from_file("src/tests/number/nan_equality.lox"), ["false", "true", "false", "true"]);
	}

	#[test]
	fn number_trailing_dot() {
		assert_eq!(run_from_file("src/tests/number/trailing_dot.lox"), ["[Line 1] Error at ';'", "Error Message: Expect property name after '.'."]);
	}

}