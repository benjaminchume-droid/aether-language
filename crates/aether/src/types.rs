use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type { Int, Float, Bool, String, Unit, Array(Box<Type>), Struct(String), Enum(String), Unknown }
impl Type {
    pub fn is_numeric(&self)->bool{matches!(self,Self::Int|Self::Float)}
    pub fn compatible_with(&self,other:&Self)->bool{match(self,other){(a,b)if a==b=>true,(a,b)if a.is_numeric()&&b.is_numeric()=>true,(Self::Unknown,_)|(_,Self::Unknown)=>true,(Self::Array(a),Self::Array(b))=>a.compatible_with(b),_=>false}}
    pub fn common_numeric(a:&Self,b:&Self)->Option<Self>{match(a,b){(Self::Float,_)|(_,Self::Float)if a.is_numeric()&&b.is_numeric()=>Some(Self::Float),(Self::Int,Self::Int)=>Some(Self::Int),_=>None}}
}
impl fmt::Display for Type{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{match self{Self::Int=>write!(f,"Int"),Self::Float=>write!(f,"Float"),Self::Bool=>write!(f,"Bool"),Self::String=>write!(f,"String"),Self::Unit=>write!(f,"Unit"),Self::Array(i)=>write!(f,"Array<{i}>"),Self::Struct(n)|Self::Enum(n)=>write!(f,"{n}"),Self::Unknown=>write!(f,"unknown")}}}
#[derive(Clone, Debug, PartialEq)]
pub struct Typed<T>{pub value:T,pub ty:Type}
