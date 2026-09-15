use crate::{hir, lexer, lower, parser, typecheck::{TypeChecker, TypedProgram}};

pub fn compile(source: &str) -> Result<TypedProgram, Vec<String>> {
    let tokens = lexer::lex(source).map_err(|e| vec![e])?;
    let ast = parser::parse(&tokens).map_err(|e| vec![e])?;
    TypeChecker::check(&ast)
}

pub fn lower_to_hir(source: &str) -> Result<hir::Program, String> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(&tokens)?;
    Ok(lower::lower(&ast))
}
