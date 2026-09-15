use aether::{lexer::lex, parser::parse, runtime::execute, typecheck::TypeChecker};

fn run(source: &str) -> String {
    let tokens = lex(source).expect("lex");
    let program = parse(&tokens).expect("parse");
    TypeChecker::check(&program).expect("type-check");
    execute(&program).expect("execute")
}

#[test]
fn array_literals_and_indexing() {
    let output = run(r#"
        let values = [10, 20, 30];
        print(values[1]);
    "#);
    assert_eq!(output, "20");
}

#[test]
fn mutable_array_element_assignment() {
    let output = run(r#"
        let mut values = [1, 2, 3];
        values[1] = 42;
        print(values[1]);
    "#);
    assert_eq!(output, "42");
}

#[test]
fn local_mutation_inside_function() {
    let output = run(r#"
        fn main() {
            let mut x = [4, 5];
            x[0] = 9;
            print(x[0]);
        }
        main();
    "#);
    assert_eq!(output, "9");
}

#[test]
fn rejects_immutable_assignment() {
    let tokens = lex("let x = 1; x = 2;").expect("lex");
    let program = parse(&tokens).expect("parse");
    let errors = TypeChecker::check(&program).expect_err("assignment should fail");
    assert!(errors.iter().any(|e| e.contains("immutable variable `x`")));
}
