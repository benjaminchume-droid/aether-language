use std::collections::HashMap;
use std::fmt;

#[derive(Clone,Debug,PartialEq)]
pub enum RuntimeValue{Int(i64),Float(f64),Bool(bool),String(String),Array(Vec<RuntimeValue>),Tuple(Vec<RuntimeValue>),Struct{name:String,fields:HashMap<String,RuntimeValue>},Enum{name:String,variant:String,payload:Vec<RuntimeValue>},Unit}
impl RuntimeValue{pub fn type_name(&self)->String{match self{Self::Int(_)=>"Int".into(),Self::Float(_)=>"Float".into(),Self::Bool(_)=>"Bool".into(),Self::String(_)=>"String".into(),Self::Array(_)=>"Array".into(),Self::Tuple(_)=>"Tuple".into(),Self::Struct{name,..}=>name.clone(),Self::Enum{name,..}=>name.clone(),Self::Unit=>"Unit".into()}}
 pub fn is_truthy(&self)->bool{match self{Self::Bool(v)=>*v,Self::Int(v)=>*v!=0,Self::Float(v)=>*v!=0.0,Self::String(v)=>!v.is_empty(),Self::Array(v)|Self::Tuple(v)=>!v.is_empty(),Self::Struct{..}|Self::Enum{..}=>true,Self::Unit=>false}}
}
impl fmt::Display for RuntimeValue{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{match self{Self::Int(v)=>write!(f,"{v}"),Self::Float(v)=>write!(f,"{v}"),Self::Bool(v)=>write!(f,"{v}"),Self::String(v)=>write!(f,"{v}"),Self::Array(xs)=>write!(f,"[{}]",xs.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")),Self::Tuple(xs)=>write!(f,"({})",xs.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")),Self::Struct{name,fields}=>{let mut e=fields.iter().map(|(k,v)|(k,v.to_string())).collect::<Vec<_>>();e.sort_by(|a,b|a.0.cmp(b.0));write!(f,"{} {{ {} }}",name,e.into_iter().map(|(k,v)|format!("{k}: {v}")).collect::<Vec<_>>().join(", "))},Self::Enum{name,variant,payload}=>if payload.is_empty(){write!(f,"{name}::{variant}")}else{write!(f,"{name}::{variant}({})",payload.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "))},Self::Unit=>write!(f,"()")}}}
