#[cfg(test)]
mod tests {
    use crate::compiler::compile;

    #[test]
    fn accepts_valid_program() {
        let src = "let answer = 40 + 2; if answer == 42 { print(answer); }";
        assert!(compile(src).is_ok());
    }

    #[test]
    fn rejects_invalid_binary_operands() {
        let src = "let bad = true + 1;";
        let errors = compile(src).expect_err("program should fail type checking");
        assert!(errors.iter().any(|e| e.contains("invalid operands")));
    }

    #[test]
    fn rejects_non_boolean_conditions() {
        let src = "if 1 { print(1); }";
        let errors = compile(src).expect_err("condition should be rejected");
        assert!(errors.iter().any(|e| e.contains("if condition must be Bool")));
    }

    #[test]
    fn rejects_wrong_arity() {
        let src = "fn one(x) { return x; } one(1, 2);";
        let errors = compile(src).expect_err("call arity should be rejected");
        assert!(errors.iter().any(|e| e.contains("expects 1 arguments")));
    }
}
