use aether::{lexer::lex, parser::parse, runtime::execute, typecheck::TypeChecker};

#[test]
fn for_in_array() {
    let source = r#"
        let values = [2, 3, 5];
        for value in values {
            print(value);
        }
    "#;
    let program = parse(&lex(source).expect("lex")).expect("parse");
    TypeChecker::check(&program).expect("type-check");
    assert_eq!(execute(&program).expect("execute"), "2\n3\n5");
}

#[test]
fn for_in_string() {
    let source = r#"
        for ch in "ae" {
            print(ch);
        }
    "#;
    let program = parse(&lex(source).expect("lex")).expect("parse");
    TypeChecker::check(&program).expect("type-check");
    assert_eq!(execute(&program).expect("execute"), "a\ne");
}
