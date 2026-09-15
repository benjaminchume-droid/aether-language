pub mod infer;
pub mod representation;

pub use infer::{Constraint, Inferencer};
pub use representation::{Ty, TypeVar};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Type;

    #[test]
    fn fresh_variables_unify() {
        let mut i = Inferencer::default();
        let a = i.fresh();
        let result = i.unify(a.clone(), Ty::Known(Type::Int)).unwrap();
        assert_eq!(result, Ty::Known(Type::Int));
        assert_eq!(i.resolve(&a), Ty::Known(Type::Int));
    }

    #[test]
    fn numeric_promotion_unifies_int_and_float() {
        let mut i = Inferencer::default();
        assert_eq!(i.unify(Ty::Known(Type::Int), Ty::Known(Type::Float)).unwrap(), Ty::Known(Type::Float));
    }
}
