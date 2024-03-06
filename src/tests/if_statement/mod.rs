#[cfg(test)]
mod tests {
	use crate::tests::run_from_file;

	#[test]
	fn if_statement_class_in_else() {
		assert_eq!(run_from_file("src/tests/if_statement/class_in_else.lox"), ["[Line 1] Error at 'class'", "Error Message: Expect expression."]);
	}

	#[test]
	fn if_statement_class_in_then() {
		assert_eq!(run_from_file("src/tests/if_statement/class_in_then.lox"), ["[Line 1] Error at 'class'", "Error Message: Expect expression."]);
	}

	#[test]
	fn if_statement_dangling_else() {
		assert_eq!(run_from_file("src/tests/if_statement/dangling_else.lox"), ["good"]);
	}

	#[test]
	fn if_statement_else() {
		assert_eq!(run_from_file("src/tests/if_statement/else.lox"), ["good", "good", "block"]);
	}

	#[test]
	fn if_statement_fun_in_else() {
		assert_eq!(run_from_file("src/tests/if_statement/fun_in_else.lox"), ["[Line 1] Error at 'fun'", "Error Message: Expect expression."]);
	}

	#[test]
	fn if_statement_fun_in_then() {
		assert_eq!(run_from_file("src/tests/if_statement/fun_in_then.lox"), ["[Line 1] Error at 'fun'", "Error Message: Expect expression."]);
	}

	#[test]
	fn if_statement_if() {
		assert_eq!(run_from_file("src/tests/if_statement/if.lox"), ["good", "block", "true"]);
	}

	#[test]
	fn if_statement_truth() {
		assert_eq!(run_from_file("src/tests/if_statement/truth.lox"), ["false", "nil", "true", "0", "empty"]);
	}

	#[test]
	fn if_statement_var_in_else() {
		assert_eq!(run_from_file("src/tests/if_statement/var_in_else.lox"), ["[Line 1] Error at 'var'", "Error Message: Expect expression."]);
	}

	#[test]
	fn if_statement_var_in_then() {
		assert_eq!(run_from_file("src/tests/if_statement/var_in_then.lox"), ["[Line 1] Error at 'var'", "Error Message: Expect expression."]);
	}

}