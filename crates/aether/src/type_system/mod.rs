pub mod infer;
pub mod representation;

pub use infer::{Constraint, Inferencer};
pub use representation::{Ty, TypeVar};
