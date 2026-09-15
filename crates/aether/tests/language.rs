use aether::{compiler::compile, lexer, parser, run};

#[test] fn lexes_core_syntax(){assert!(lexer::lex("let x: Int = 1 + 2;").unwrap().len()>1);}
#[test] fn parses_and_runs_arithmetic(){assert_eq!(run("let x: Int = 2 * (3 + 4); print(x);").unwrap(),"14");}
#[test] fn conditionals_and_comparisons(){assert_eq!(run("let x: Int = 7; if x > 3 { print(\"yes\"); } else { print(\"no\"); }").unwrap(),"yes");}
#[test] fn typed_functions_and_returns(){assert_eq!(run("fn add(a: Int, b: Int) -> Int { return a + b; } print(add(20, 22));").unwrap(),"42");}
#[test] fn rejects_wrong_argument_type(){assert!(compile("fn add(a: Int) -> Int { return a; } add(\"x\");").is_err());}
#[test] fn rejects_wrong_variable_annotation(){assert!(compile("let x: Bool = 42;").is_err());}
#[test] fn supports_nested_array_types(){assert_eq!(run("let xs: Array<Int> = [1, 2, 3]; print(xs[1]);").unwrap(),"2");}
#[test] fn rejects_bad_return_type(){assert!(compile("fn value() -> String { return 42; }").is_err());}
#[test] fn structs_execute(){assert_eq!(run("struct User { name: String, age: Int } let mut u: User = User { name: \"Ada\", age: 36 }; u.age = 37; print(u.name, u.age);").unwrap(),"Ada 37");}
#[test] fn enums_and_match_execute(){assert_eq!(run("enum Color { Red, Green, Blue } let c: Color = Color::Green; match c { Color::Red => { print(\"red\"); }, Color::Green => { print(\"green\"); }, Color::Blue => { print(\"blue\"); } }").unwrap(),"green");}
#[test] fn match_wildcard_executes(){assert_eq!(run("enum Color { Red, Green } let c = Color::Red; match c { Color::Green => { print(\"green\"); }, _ => { print(\"other\"); } }").unwrap(),"other");}
#[test] fn rejects_non_exhaustive_match(){assert!(compile("enum Color { Red, Green } let c = Color::Red; match c { Color::Red => { print(1); } }").is_err());}
#[test] fn rejects_unknown_variant(){assert!(compile("enum Color { Red, Green } let c = Color::Red; match c { Color::Blue => { print(1); }, _ => { print(2); } }").is_err());}
#[test] fn payload_enums_and_pattern_bindings(){assert_eq!(run("enum Message { Quit, Text(String), Count(Int) } let m: Message = Message::Text(\"hello\"); match m { Message::Quit => { print(\"quit\"); }, Message::Text(text) => { print(text); }, Message::Count(n) => { print(n); } }").unwrap(),"hello");}
#[test] fn rejects_bad_enum_payload(){assert!(compile("enum Message { Text(String) } let m = Message::Text(42);").is_err());}
#[test] fn rejects_bad_pattern_binding_count(){assert!(compile("enum Message { Text(String) } let m = Message::Text(\"x\"); match m { Message::Text(a, b) => { print(a); }, }").is_err());}
#[test] fn collections_and_mutation(){assert_eq!(run("let mut xs: Array<Int> = [1, 2, 3]; xs[1] = 9; for x in xs { print(x); }").unwrap(),"1\n9\n3");}
#[test] fn loop_break_and_continue(){assert_eq!(run("let mut x: Int = 0; loop { x = x + 1; if x == 2 { continue; } if x == 4 { break; } print(x); }").unwrap(),"1\n3");}
#[test] fn rejects_break_outside_loop(){assert!(run("break;").is_err());}
#[test] fn rejects_continue_outside_loop(){assert!(run("continue;").is_err());}
#[test] fn parser_accepts_program(){parser::parse(&lexer::lex("fn main() { print(true); }").unwrap()).unwrap();}
