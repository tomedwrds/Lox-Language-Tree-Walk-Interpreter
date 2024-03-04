#[cfg(test)]
mod tests {
	use crate::tests::run_from_file;

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
	fn variable_uninitialized() {
		assert_eq!(run_from_file("src/tests/variable/uninitialized.lox"), ["nil"]);
	}

	#[test]
	fn variable_unreached_undefined() {
		assert_eq!(run_from_file("src/tests/variable/unreached_undefined.lox"), ["ok"]);
	}

	#[test]
	fn variable_use_global_in_initializer() {
		assert_eq!(run_from_file("src/tests/variable/use_global_in_initializer.lox"), ["value"]);
	}

}