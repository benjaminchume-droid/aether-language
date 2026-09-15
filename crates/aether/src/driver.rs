use crate::{lexer, lower, parser};

pub fn front_end(source: &str) -> Result<crate::hir::Program, String> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(&tokens)?;
    Ok(lower::lower(&ast))
}
