use aether::{lexer, parser, run};

#[test]
fn lexes_core_syntax() {
    let tokens = lexer::lex("let x = 1 + 2;").unwrap();
    assert!(tokens.len() > 1);
}

#[test]
fn parses_and_runs_arithmetic() {
    assert_eq!(run("let x = 2 * (3 + 4); print(x);").unwrap(), "14");
}

#[test]
fn conditionals_and_comparisons() {
    let src = "let x = 7; if x > 3 { print(\"yes\"); } else { print(\"no\"); }";
    assert_eq!(run(src).unwrap(), "yes");
}

#[test]
fn functions_and_return() {
    let src = "fn add(a, b) { return a + b; } print(add(20, 22));";
    assert_eq!(run(src).unwrap(), "42");
}

#[test]
fn collections_and_mutation() {
    let src = "let mut xs = [1, 2, 3]; xs[1] = 9; for x in xs { print(x); }";
    assert_eq!(run(src).unwrap(), "1\n9\n3");
}

#[test]
fn loop_break_and_continue() {
    let src = "let mut x = 0; loop { x = x + 1; if x == 2 { continue; } if x == 4 { break; } print(x); }";
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
