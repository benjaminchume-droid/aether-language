use crate::types::Type;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeVar(pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ty {
    Known(Type),
    Variable(TypeVar),
    Function { params: Vec<Ty>, return_type: Box<Ty> },
}

impl Ty {
    pub const fn unit() -> Self { Self::Known(Type::Unit) }
    pub const fn variable(id: u32) -> Self { Self::Variable(TypeVar(id)) }
}
