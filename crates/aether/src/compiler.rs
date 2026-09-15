use crate::{lexer, parser, typecheck::{TypeChecker, TypedProgram}};

pub fn compile(source: &str) -> Result<TypedProgram, Vec<String>> {
    let tokens = lexer::lex(source).map_err(|e| vec![e])?;
    let program = parser::parse(&tokens).map_err(|e| vec![e])?;
    TypeChecker::check(&program)
}
