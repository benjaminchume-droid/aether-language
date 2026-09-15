use std::collections::HashMap;
use super::representation::{Ty, TypeVar};
use crate::types::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constraint { pub left: Ty, pub right: Ty }

#[derive(Default)]
pub struct Inferencer { substitutions: HashMap<TypeVar, Ty>, next: u32 }

impl Inferencer {
    pub fn fresh(&mut self) -> Ty { let v=TypeVar(self.next); self.next+=1; Ty::Variable(v) }
    pub fn resolve(&self, ty: &Ty) -> Ty {
        match ty { Ty::Variable(v) => self.substitutions.get(v).map(|t| self.resolve(t)).unwrap_or_else(|| ty.clone()), Ty::Function{params,return_type}=>Ty::Function{params:params.iter().map(|t|self.resolve(t)).collect(),return_type:Box::new(self.resolve(return_type))}, Ty::Known(t)=>Ty::Known(t.clone()) }
    }
    pub fn unify(&mut self, a: Ty, b: Ty) -> Result<Ty, String> {
        let a=self.resolve(&a); let b=self.resolve(&b);
        match (a,b) {
            (Ty::Variable(v), t) | (t, Ty::Variable(v)) => { if self.occurs(v,&t){return Err("recursive type detected".into())} self.substitutions.insert(v,t.clone()); Ok(t) },
            (Ty::Known(x), Ty::Known(y)) if x==y => Ok(Ty::Known(x)),
            (Ty::Known(Type::Int), Ty::Known(Type::Float)) | (Ty::Known(Type::Float), Ty::Known(Type::Int)) => Ok(Ty::Known(Type::Float)),
            (Ty::Function{params:a,return_type:r1}, Ty::Function{params:b,return_type:r2}) if a.len()==b.len() => { let ps=a.into_iter().zip(b).map(|(x,y)|self.unify(x,y)).collect::<Result<Vec<_>,_>>()?; let r=self.unify(*r1,*r2)?; Ok(Ty::Function{params:ps,return_type:Box::new(r)}) },
            (x,y)=>Err(format!("cannot unify {x:?} with {y:?}")),
        }
    }
    fn occurs(&self, needle: TypeVar, ty: &Ty) -> bool { match ty { Ty::Variable(v)=>*v==needle, Ty::Function{params,return_type}=>params.iter().any(|t|self.occurs(needle,t))||self.occurs(needle,return_type), Ty::Known(_)=>false } }
}
