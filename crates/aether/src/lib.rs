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

pub fn run(source: &str) -> Result<String, String> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(&tokens)?;
    typecheck::TypeChecker::check(&ast).map_err(|errors| errors.join("\n"))?;
    runtime::execute(&ast)
}
