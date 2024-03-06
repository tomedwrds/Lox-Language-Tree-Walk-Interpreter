#[cfg(test)]
mod tests {
	use crate::tests::run_from_file;

	#[test]
	fn for_loop_class_in_body() {
		assert_eq!(run_from_file("src/tests/for_loop/class_in_body.lox"), ["[Line 1] Error at 'class'", "Error Message: Expect expression."]);
	}

	#[test]
	fn for_loop_closure_in_body() {
		assert_eq!(run_from_file("src/tests/for_loop/closure_in_body.lox"), ["4", "1", "4", "2", "4", "3"]);
	}

	#[test]
	fn for_loop_fun_in_body() {
		assert_eq!(run_from_file("src/tests/for_loop/fun_in_body.lox"), ["[Line 1] Error at 'fun'", "Error Message: Expect expression."]);
	}

	#[test]
	fn for_loop_return_closure() {
		assert_eq!(run_from_file("src/tests/for_loop/return_closure.lox"), ["i"]);
	}

	#[test]
	fn for_loop_return_inside() {
		assert_eq!(run_from_file("src/tests/for_loop/return_inside.lox"), ["i"]);
	}

	#[test]
	fn for_loop_scope() {
		assert_eq!(run_from_file("src/tests/for_loop/scope.lox"), ["0", "-1", "after", "0"]);
	}

	#[test]
	fn for_loop_statement_condition() {
		assert_eq!(run_from_file("src/tests/for_loop/statement_condition.lox"), ["[Line 1] Error at '{'", "Error Message: Expect expression.","[Line 1] Error at ')'", "Error Message: Expect ';' after expression."]);
	}

	#[test]
	fn for_loop_statement_increment() {
		assert_eq!(run_from_file("src/tests/for_loop/statement_increment.lox"), ["[Line 1] Error at '{'", "Error Message: Expect expression."]);
	}

	#[test]
	fn for_loop_statement_initializer() {
		assert_eq!(run_from_file("src/tests/for_loop/statement_initializer.lox"), ["[Line 1] Error at '{'", "Error Message: Expect expression.","[Line 1] Error at ')'", "Error Message: Expect ';' after expression."]);
	}

	#[test]
	fn for_loop_syntax() {
		assert_eq!(run_from_file("src/tests/for_loop/syntax.lox"), ["1", "2", "3", "0", "1", "2", "done", "0", "1", "0", "1", "2", "0", "1"]);
	}

	#[test]
	fn for_loop_var_in_body() {
		assert_eq!(run_from_file("src/tests/for_loop/var_in_body.lox"), ["[Line 1] Error at 'var'", "Error Message: Expect expression."]);
	}

}