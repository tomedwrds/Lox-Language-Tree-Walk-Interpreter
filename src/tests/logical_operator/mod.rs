#[cfg(test)]
mod tests {
	use crate::tests::run_from_file;

	#[test]
	fn logical_operator_and() {
		assert_eq!(run_from_file("src/tests/logical_operator/and.lox"), ["false", "1", "false", "true", "3", "true", "false"]);
	}

	#[test]
	fn logical_operator_and_truth() {
		assert_eq!(run_from_file("src/tests/logical_operator/and_truth.lox"), ["false", "nil", "ok", "ok", "ok"]);
	}

	#[test]
	fn logical_operator_or() {
		assert_eq!(run_from_file("src/tests/logical_operator/or.lox"), ["1", "1", "true", "false", "false", "false", "true"]);
	}

	#[test]
	fn logical_operator_or_truth() {
		assert_eq!(run_from_file("src/tests/logical_operator/or_truth.lox"), ["ok", "ok", "true", "0", "s"]);
	}

}