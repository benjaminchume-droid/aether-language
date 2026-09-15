use crate::ast::Program;
use crate::typecheck::TypeChecker;
use crate::types::Type;

#[derive(Clone, Debug)]
pub struct Symbol { pub ty: Type, pub mutable: bool }

#[derive(Default)]
pub struct Analyzer;

impl Analyzer {
    pub fn analyze(program: &Program) -> Result<(), Vec<String>> {
        TypeChecker::check(program).map(|_|()).map_err(|errors| errors)
    }
}
