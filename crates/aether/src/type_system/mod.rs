use crate::types::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeVar(pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ty {
    Known(Type),
    Variable(TypeVar),
    Function { params: Vec<Ty>, return_type: Box<Ty> },
}

impl Ty {
    pub fn unit() -> Self { Self::Known(Type::Unit) }
    pub fn unknown() -> Self { Self::Variable(TypeVar(0)) }
}
