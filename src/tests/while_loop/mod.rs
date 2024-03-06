#[cfg(test)]
mod tests {
	use crate::tests::run_from_file;

	#[test]
	fn while_loop_class_in_body() {
		assert_eq!(run_from_file("src/tests/while_loop/class_in_body.lox"), ["[Line 1] Error at 'class'", "Error Message: Expect expression."]);
	}

	#[test]
	fn while_loop_closure_in_body() {
		assert_eq!(run_from_file("src/tests/while_loop/closure_in_body.lox"), ["1", "2", "3"]);
	}

	#[test]
	fn while_loop_fun_in_body() {
		assert_eq!(run_from_file("src/tests/while_loop/fun_in_body.lox"), ["[Line 1] Error at 'fun'", "Error Message: Expect expression."]);
	}

	#[test]
	fn while_loop_return_closure() {
		assert_eq!(run_from_file("src/tests/while_loop/return_closure.lox"), ["i"]);
	}

	#[test]
	fn while_loop_return_inside() {
		assert_eq!(run_from_file("src/tests/while_loop/return_inside.lox"), ["i"]);
	}

	#[test]
	fn while_loop_syntax() {
		assert_eq!(run_from_file("src/tests/while_loop/syntax.lox"), ["1", "2", "3", "0", "1", "2"]);
	}

	#[test]
	fn while_loop_var_in_body() {
		assert_eq!(run_from_file("src/tests/while_loop/var_in_body.lox"), ["[Line 1] Error at 'var'", "Error Message: Expect expression."]);
	}

}