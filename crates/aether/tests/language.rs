use aether::{lexer, parser, run};

#[test]
fn lexes_core_syntax() {
    let tokens = lexer::lex("let x: Int = 1 + 2;").unwrap();
    assert!(tokens.len() > 1);
}

#[test]
fn parses_and_runs_arithmetic() {
    assert_eq!(run("let x: Int = 2 * (3 + 4); print(x);").unwrap(), "14");
}

#[test]
fn conditionals_and_comparisons() {
    let src = "let x: Int = 7; if x > 3 { print(\"yes\"); } else { print(\"no\"); }";
    assert_eq!(run(src).unwrap(), "yes");
}

#[test]
fn typed_functions_and_returns() {
    let src = "fn add(a: Int, b: Int) -> Int { return a + b; } print(add(20, 22));";
    assert_eq!(run(src).unwrap(), "42");
}

#[test]
fn rejects_wrong_argument_type() {
    let src = "fn add(a: Int, b: Int) -> Int { return a + b; } print(add(20, \"22\"));";
    assert!(run(src).is_err());
}

#[test]
fn rejects_wrong_variable_annotation() {
    assert!(run("let x: Bool = 42;").is_err());
}

#[test]
fn supports_nested_array_types() {
    assert_eq!(run("let xs: Array<Int> = [1, 2, 3]; print(xs[1]);").unwrap(), "2");
}

#[test]
fn rejects_bad_return_type() {
    assert!(run("fn value() -> String { return 42; }").is_err());
}

#[test]
fn collections_and_mutation() {
    let src = "let mut xs: Array<Int> = [1, 2, 3]; xs[1] = 9; for x in xs { print(x); }";
    assert_eq!(run(src).unwrap(), "1\n9\n3");
}

#[test]
fn loop_break_and_continue() {
    let src = "let mut x: Int = 0; loop { x = x + 1; if x == 2 { continue; } if x == 4 { break; } print(x); }";
    assert_eq!(run(src).unwrap(), "1\n3");
}

#[test]
fn rejects_break_outside_loop() {
    assert!(run("break;").is_err());
}

#[test]
fn rejects_continue_outside_loop() {
    assert!(run("continue;").is_err());
}

#[test]
fn parser_accepts_program() {
    let tokens = lexer::lex("fn main() { print(true); }").unwrap();
    parser::parse(&tokens).unwrap();
}
