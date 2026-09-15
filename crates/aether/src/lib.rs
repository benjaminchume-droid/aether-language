pub mod ast;
pub mod lexer;
pub mod parser;
pub mod runtime;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AetherError {
    #[error("lex error: {0}")]
    Lex(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("runtime error: {0}")]
    Runtime(String),
}

pub fn run(source: &str) -> Result<String, AetherError> {
    let tokens = lexer::lex(source).map_err(AetherError::Lex)?;
    let program = parser::parse(&tokens).map_err(AetherError::Parse)?;
    runtime::execute(&program).map_err(AetherError::Runtime)
}
