pub mod ast;
pub mod compiler;
pub mod diagnostics;
pub mod driver;
pub mod hir;
pub mod lexer;
pub mod lower;
pub mod parser;
pub mod prelude;
pub mod runtime;
pub mod runtime_types;
pub mod semantic;
pub mod type_system;
pub mod typecheck;
pub mod types;

pub fn run(source:&str)->Result<String,String>{let tokens=lexer::lex(source)?;let ast=parser::parse(&tokens)?;if ast.items.iter().any(|i|matches!(i,ast::Item::Import(_))){return Err("imports require run_file so relative paths can be resolved".into())}typecheck::TypeChecker::check(&ast).map_err(|errors|errors.join("\n"))?;runtime::execute(&ast)}
pub fn run_file(path:impl AsRef<std::path::Path>)->Result<String,String>{let ast=compiler::load_file(path)?;typecheck::TypeChecker::check(&ast).map_err(|errors|errors.join("\n"))?;runtime::execute(&ast)}
