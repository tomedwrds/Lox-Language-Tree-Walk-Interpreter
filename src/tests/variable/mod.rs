#[cfg(test)]
mod tests {
	use crate::tests::run_from_file;

	#[test]
	fn variable_collide_with_parameter() {
		assert_eq!(run_from_file("src/tests/variable/collide_with_parameter.lox"), ["Line 2] Error at 'a'", "Error message: Already a variable with this name in this scope."]);
	}

	#[test]
	fn variable_duplicate_local() {
		assert_eq!(run_from_file("src/tests/variable/duplicate_local.lox"), ["Line 3] Error at 'a'", "Error message: Already a variable with this name in this scope."]);
	}

	#[test]
	fn variable_duplicate_parameter() {
		assert_eq!(run_from_file("src/tests/variable/duplicate_parameter.lox"), ["Line 2] Error at 'arg'", "Error message: Already a variable with this name in this scope."]);
	}

	#[test]
	fn variable_early_bound() {
		assert_eq!(run_from_file("src/tests/variable/early_bound.lox"), ["outer", "outer"]);
	}

	#[test]
	fn variable_in_middle_of_block() {
		assert_eq!(run_from_file("src/tests/variable/in_middle_of_block.lox"), ["a", "a b", "a c", "a b d"]);
	}

	#[test]
	fn variable_in_nested_block() {
		assert_eq!(run_from_file("src/tests/variable/in_nested_block.lox"), ["outer"]);
	}

	#[test]
	fn variable_local_from_method() {
		assert_eq!(run_from_file("src/tests/variable/local_from_method.lox"), ["variable"]);
	}

	#[test]
	fn variable_redeclare_global() {
		assert_eq!(run_from_file("src/tests/variable/redeclare_global.lox"), ["nil"]);
	}

	#[test]
	fn variable_redefine_global() {
		assert_eq!(run_from_file("src/tests/variable/redefine_global.lox"), ["2"]);
	}

	#[test]
	fn variable_scope_reuse_in_different_blocks() {
		assert_eq!(run_from_file("src/tests/variable/scope_reuse_in_different_blocks.lox"), ["first", "second"]);
	}

	#[test]
	fn variable_shadow_and_local() {
		assert_eq!(run_from_file("src/tests/variable/shadow_and_local.lox"), ["outer", "inner"]);
	}

	#[test]
	fn variable_shadow_global() {
		assert_eq!(run_from_file("src/tests/variable/shadow_global.lox"), ["shadow", "global"]);
	}

	#[test]
	fn variable_shadow_local() {
		assert_eq!(run_from_file("src/tests/variable/shadow_local.lox"), ["shadow", "local"]);
	}

	#[test]
	fn variable_undefined_global() {
		assert_eq!(run_from_file("src/tests/variable/undefined_global.lox"), ["Line 1] Runtime Var Error", "Error message: Undefined variable 'notDefined'."]);
	}

	#[test]
	fn variable_undefined_local() {
		assert_eq!(run_from_file("src/tests/variable/undefined_local.lox"), ["Line 2] Runtime Var Error", "Error message: Undefined variable 'notDefined'."]);
	}

	#[test]
	fn variable_uninitialized() {
		assert_eq!(run_from_file("src/tests/variable/uninitialized.lox"), ["nil"]);
	}

	#[test]
	fn variable_unreached_undefined() {
		assert_eq!(run_from_file("src/tests/variable/unreached_undefined.lox"), ["ok"]);
	}

	#[test]
	fn variable_use_false_as_var() {
		assert_eq!(run_from_file("src/tests/variable/use_false_as_var.lox"), ["Line 1] Error at 'false'", "Error message: Expect variable name."]);
	}

	#[test]
	fn variable_use_global_in_initializer() {
		assert_eq!(run_from_file("src/tests/variable/use_global_in_initializer.lox"), ["value"]);
	}

	#[test]
	fn variable_use_local_in_initializer() {
		assert_eq!(run_from_file("src/tests/variable/use_local_in_initializer.lox"), ["Line 3] Error at 'a'", "Error message: Can't read local variable in its own initializer."]);
	}

	#[test]
	fn variable_use_nil_as_var() {
		assert_eq!(run_from_file("src/tests/variable/use_nil_as_var.lox"), ["Line 1] Error at 'nil'", "Error message: Expect variable name."]);
	}

	#[test]
	fn variable_use_this_as_var() {
		assert_eq!(run_from_file("src/tests/variable/use_this_as_var.lox"), ["Line 1] Error at 'this'", "Error message: Expect variable name."]);
	}

}