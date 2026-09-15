use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Unit,
    Unknown,
}

impl Type {
    pub fn is_numeric(&self) -> bool { matches!(self, Self::Int | Self::Float) }

    pub fn compatible_with(&self, other: &Self) -> bool {
        self == other || (self.is_numeric() && other.is_numeric()) || matches!((self, other), (Self::Unknown, _) | (_, Self::Unknown))
    }

    pub fn common_numeric(a: &Self, b: &Self) -> Option<Self> {
        match (a, b) {
            (Self::Float, _) | (_, Self::Float) => Some(Self::Float),
            (Self::Int, Self::Int) => Some(Self::Int),
            _ => None,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self { Self::Int => "Int", Self::Float => "Float", Self::Bool => "Bool", Self::String => "String", Self::Unit => "Unit", Self::Unknown => "unknown" };
        write!(f, "{name}")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Typed<T> { pub value: T, pub ty: Type }
