use crate::ast::*;
use crate::types::Type;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)] pub struct TypedProgram { pub items: Vec<TypedItem> }
#[derive(Clone, Debug)] pub enum TypedItem {
    Let { name:String, mutable:bool, annotation:Option<Type>, value:TypedExpr },
    Fn { name:String, params:Vec<Param>, return_type:Type, body:Vec<TypedStmt> },
    Struct { name:String, fields:Vec<Field> },
    Enum { name:String, variants:Vec<EnumVariant> },
    Stmt(TypedStmt),
}
#[derive(Clone, Debug)] pub enum TypedStmt {
    Expr(TypedExpr), Let { name:String, mutable:bool, annotation:Option<Type>, value:TypedExpr },
    Assign { target:TypedExpr, value:TypedExpr }, Return(Option<TypedExpr>),
    If { condition:TypedExpr, then_branch:Vec<TypedStmt>, else_branch:Vec<TypedStmt> },
    While { condition:TypedExpr, body:Vec<TypedStmt> }, For { name:String, iterable:TypedExpr, body:Vec<TypedStmt> },
    Loop { body:Vec<TypedStmt> }, Match { value:TypedExpr, arms:Vec<MatchArm> }, Break, Continue,
}
#[derive(Clone, Debug)] pub struct TypedExpr { pub expr:Expr, pub ty:Type }

#[derive(Default)] pub struct TypeChecker {
    scopes:Vec<HashMap<String,Type>>, mutability:Vec<HashMap<String,bool>>,
    functions:HashMap<String,(Vec<Type>,Type)>, structs:HashMap<String,HashMap<String,Type>>,
    enums:HashMap<String,HashMap<String,Vec<Type>>>, loop_depth:usize, current_return:Option<Type>,
}
impl TypeChecker {
    pub fn check(program:&Program)->Result<TypedProgram,Vec<String>> {
        let mut c=Self::default(); c.push_scope();
        for i in &program.items { match i {
            Item::Fn{name,params,return_type,..}=>{c.functions.insert(name.clone(),(params.iter().map(|p|p.ty.clone().unwrap_or(Type::Unknown)).collect(),return_type.clone().unwrap_or(Type::Unknown)));},
            Item::Struct{name,fields}=>{c.structs.insert(name.clone(),fields.iter().map(|f|(f.name.clone(),f.ty.clone())).collect());},
            Item::Enum{name,variants}=>{c.enums.insert(name.clone(),variants.iter().map(|v|(v.name.clone(),v.payload.clone())).collect());}, _=>{}
        }}
        let mut errors=Vec::new(); let mut out=Vec::new();
        for i in &program.items { match c.item(i){Ok(x)=>out.push(x),Err(es)=>errors.extend(es)} }
        if errors.is_empty(){Ok(TypedProgram{items:out})}else{Err(errors)}
    }
    fn push_scope(&mut self){self.scopes.push(HashMap::new());self.mutability.push(HashMap::new());}
    fn pop_scope(&mut self){self.scopes.pop();self.mutability.pop();}
    fn define(&mut self,n:&str,t:Type,m:bool)->Result<(),String>{let s=self.scopes.last_mut().unwrap();if s.contains_key(n){return Err(format!("duplicate definition of `{n}`"))}s.insert(n.into(),t);self.mutability.last_mut().unwrap().insert(n.into(),m);Ok(())}
    fn lookup(&self,n:&str)->Option<Type>{self.scopes.iter().rev().find_map(|s|s.get(n).cloned())}
    fn is_mutable(&self,n:&str)->bool{self.mutability.iter().rev().find_map(|s|s.get(n).copied()).unwrap_or(false)}
    fn bind(&self,a:&Option<Type>,actual:&Type,n:&str)->Result<Type,String>{match a{Some(t)if!t.compatible_with(actual)=>Err(format!("type annotation for `{n}` expects {t}, found {actual}")),Some(t)=>Ok(t.clone()),None=>Ok(actual.clone())}}
    fn item(&mut self,i:&Item)->Result<TypedItem,Vec<String>>{match i {
        Item::Enum{name,variants}=>{if variants.is_empty(){return Err(vec![format!("enum `{name}` must have at least one variant")])}let mut seen=HashSet::new();for v in variants{if !seen.insert(&v.name){return Err(vec![format!("enum `{name}` contains duplicate variant `{}`",v.name)])}}Ok(TypedItem::Enum{name:name.clone(),variants:variants.clone()})},
        Item::Struct{name,fields}=>{let mut seen=HashSet::new();for f in fields{if !seen.insert(&f.name){return Err(vec![format!("struct `{name}` contains duplicate field `{}`",f.name)])}}Ok(TypedItem::Struct{name:name.clone(),fields:fields.clone()})},
        Item::Let{name,mutable,annotation,value}=>self.expr(value).and_then(|v|{let t=self.bind(annotation,&v.ty,name)?;self.define(name,t,*mutable)?;Ok(TypedItem::Let{name:name.clone(),mutable:*mutable,annotation:annotation.clone(),value:v})}).map_err(|e|vec![e]),
        Item::Fn{name,params,return_type,body}=>{self.define(name,Type::Unknown,false).map_err(|e|vec![e])?;self.push_scope();let mut errors=Vec::new();for p in params{if let Err(e)=self.define(&p.name,p.ty.clone().unwrap_or(Type::Unknown),true){errors.push(e)}}let old=self.current_return.take();self.current_return=return_type.clone();let mut b=Vec::new();for s in body{match self.stmt(s){Ok(x)=>b.push(x),Err(es)=>errors.extend(es)}}let inferred=self.current_return.clone().unwrap_or(Type::Unit);self.current_return=old;self.pop_scope();if errors.is_empty(){self.functions.insert(name.clone(),(params.iter().map(|p|p.ty.clone().unwrap_or(Type::Unknown)).collect(),inferred.clone()));Ok(TypedItem::Fn{name:name.clone(),params:params.clone(),return_type:inferred,body:b})}else{Err(errors)}},
        Item::Stmt(s)=>self.stmt(s).map(TypedItem::Stmt)
    }}
    fn stmt(&mut self,s:&Stmt)->Result<TypedStmt,Vec<String>>{match s {
        Stmt::Expr(e)=>self.expr(e).map(TypedStmt::Expr).map_err(|e|vec![e]),
        Stmt::Let{name,mutable,annotation,value}=>self.expr(value).and_then(|v|{let t=self.bind(annotation,&v.ty,name)?;self.define(name,t,*mutable)?;Ok(TypedStmt::Let{name:name.clone(),mutable:*mutable,annotation:annotation.clone(),value:v})}).map_err(|e|vec![e]),
        Stmt::Assign{target,value}=>{let t=self.expr(target).map_err(|e|vec![e])?;let v=self.expr(value).map_err(|e|vec![e])?;if let Expr::Ident(n)=&t.expr{if !self.is_mutable(n){return Err(vec![format!("cannot assign to immutable variable `{n}`")])}}if !t.ty.compatible_with(&v.ty){return Err(vec![format!("cannot assign {} to {}",v.ty,t.ty)])}Ok(TypedStmt::Assign{target:t,value:v})},
        Stmt::Return(e)=>{let value=e.as_ref().map(|x|self.expr(x)).transpose().map_err(|e|vec![e])?;let actual=value.as_ref().map(|x|x.ty.clone()).unwrap_or(Type::Unit);match &mut self.current_return{Some(expected)if*expected!=Type::Unknown=>{if !expected.compatible_with(&actual){return Err(vec![format!("function must return {}, found {}",expected,actual)])}},Some(expected)=>*expected=actual,None=>self.current_return=Some(actual)}Ok(TypedStmt::Return(value))},
        Stmt::If{condition,then_branch,else_branch}=>{let c=self.expr(condition).map_err(|e|vec![e])?;if c.ty!=Type::Bool&&c.ty!=Type::Unknown{return Err(vec![format!("if condition must be Bool, found {}",c.ty)])}self.push_scope();let t=self.block(then_branch);self.pop_scope();self.push_scope();let e=self.block(else_branch);self.pop_scope();Ok(TypedStmt::If{condition:c,then_branch:t?,else_branch:e?})},
        Stmt::While{condition,body}=>{let c=self.expr(condition).map_err(|e|vec![e])?;if c.ty!=Type::Bool&&c.ty!=Type::Unknown{return Err(vec![format!("while condition must be Bool, found {}",c.ty)])}self.push_scope();self.loop_depth+=1;let b=self.block(body);self.loop_depth-=1;self.pop_scope();Ok(TypedStmt::While{condition:c,body:b?})},
        Stmt::For{name,iterable,body}=>{let it=self.expr(iterable).map_err(|e|vec![e])?;let inner=match it.ty{Type::Array(x)=>*x,Type::String=>Type::String,_=>return Err(vec![format!("for-in requires an Array or String, found {}",it.ty)])};self.push_scope();self.define(name,&inner,false).map_err(|e|vec![e])?;self.loop_depth+=1;let b=self.block(body);self.loop_depth-=1;self.pop_scope();Ok(TypedStmt::For{name:name.clone(),iterable:it,body:b?})},
        Stmt::Loop{body}=>{self.push_scope();self.loop_depth+=1;let b=self.block(body);self.loop_depth-=1;self.pop_scope();Ok(TypedStmt::Loop{body:b?})},
        Stmt::Match{value,arms}=>{let v=self.expr(value).map_err(|e|vec![e])?;let Type::Enum(en)=v.ty.clone()else{return Err(vec!["match requires an enum value".into()])};let variants=self.enums.get(&en).cloned().ok_or_else(||format!("unknown enum `{en}`"))?;let mut seen=HashSet::new();let mut wildcard=false;let mut typed=Vec::new();for arm in arms{self.push_scope();match &arm.pattern{Pattern::Wildcard=>wildcard=true,Pattern::Enum{enum_name,variant,bindings}=>{if enum_name!=&en{self.pop_scope();return Err(vec![format!("pattern enum `{enum_name}` does not match `{en}`")])}let payload=variants.get(variant).ok_or_else(||format!("enum `{en}` has no variant `{variant}`"))?;if !seen.insert(variant.clone()){self.pop_scope();return Err(vec![format!("duplicate match arm for `{en}::{variant}`")])}if payload.len()!=bindings.len(){self.pop_scope();return Err(vec![format!("variant `{en}::{variant}` carries {} values, pattern binds {}",payload.len(),bindings.len())])}for (name,ty) in bindings.iter().zip(payload.iter()){self.define(name,ty.clone(),false).map_err(|e|vec![e])?}}let body=self.block(&arm.body);self.pop_scope();typed.push(MatchArm{pattern:arm.pattern.clone(),body:body?});}if !wildcard&&seen.len()!=variants.len(){return Err(vec![format!("non-exhaustive match on `{en}`; add `_` or cover every variant")])}Ok(TypedStmt::Match{value:v,arms:typed})},
        Stmt::Break=>if self.loop_depth==0{Err(vec!["break is only valid inside a loop".into()])}else{Ok(TypedStmt::Break)},
        Stmt::Continue=>if self.loop_depth==0{Err(vec!["continue is only valid inside a loop".into()])}else{Ok(TypedStmt::Continue)},
    }}
    fn block(&mut self,b:&[Stmt])->Result<Vec<TypedStmt>,Vec<String>>{let mut out=Vec::new();let mut errors=Vec::new();for s in b{match self.stmt(s){Ok(x)=>out.push(x),Err(es)=>errors.extend(es)}}if errors.is_empty(){Ok(out)}else{Err(errors)}}
    fn expr(&mut self,e:&Expr)->Result<TypedExpr,String>{let ty=match e{
        Expr::Int(_)=>Type::Int,Expr::Float(_)=>Type::Float,Expr::Bool(_)=>Type::Bool,Expr::Str(_)=>Type::String,
        Expr::Array(v)=>{let mut inner=Type::Unknown;for x in v{let t=self.expr(x)?.ty;if inner==Type::Unknown{inner=t}else if !inner.compatible_with(&t){return Err(format!("array elements must have compatible types, found {} and {}",inner,t))}}Type::Array(Box::new(inner))},
        Expr::Ident(n)=>self.lookup(n).ok_or_else(||format!("unknown identifier `{n}`"))?,
        Expr::StructInit{name,fields}=>{let spec=self.structs.get(name).cloned().ok_or_else(||format!("unknown struct `{name}`"))?;if fields.len()!=spec.len(){return Err(format!("struct `{name}` expects {} fields, got {}",spec.len(),fields.len()))}let mut seen=HashSet::new();for(field,value)in fields{let expected=spec.get(field).ok_or_else(||format!("unknown field `{field}` on struct `{name}`"))?;if !seen.insert(field){return Err(format!("field `{field}` initialized more than once"))}let actual=self.expr(value)?.ty;if !expected.compatible_with(&actual){return Err(format!("field `{field}` expects {}, found {}",expected,actual))}}Type::Struct(name.clone())},
        Expr::Member{target,field}=>{let t=self.expr(target)?;let Type::Struct(n)=t.ty else{return Err(format!("cannot access field `{field}` on {}",t.ty))};self.structs.get(&n).and_then(|f|f.get(field)).cloned().ok_or_else(||format!("struct `{n}` has no field `{field}`"))?},
        Expr::EnumValue{enum_name,variant,payload}=>{let variants=self.enums.get(enum_name).ok_or_else(||format!("unknown enum `{enum_name}`"))?;let expected=variants.get(variant).ok_or_else(||format!("enum `{enum_name}` has no variant `{variant}`"))?;if expected.len()!=payload.len(){return Err(format!("variant `{enum_name}::{variant}` expects {} values, got {}",expected.len(),payload.len()))}for(i,value)in payload.iter().enumerate(){let actual=self.expr(value)?.ty;if !expected[i].compatible_with(&actual){return Err(format!("payload {} of `{enum_name}::{variant}` expects {}, found {}",i+1,expected[i],actual))}}Type::Enum(enum_name.clone())},
        Expr::Index{target,index}=>{let t=self.expr(target)?;let i=self.expr(index)?;if i.ty!=Type::Int&&i.ty!=Type::Unknown{return Err(format!("collection index must be Int, found {}",i.ty))}match t.ty{Type::Array(inner)=>*inner,Type::String=>Type::String,_=>return Err(format!("cannot index {}",t.ty))}},
        Expr::Unary{op,expr}=>{let t=self.expr(expr)?.ty;match op{UnaryOp::Neg if t.is_numeric()=>t,UnaryOp::Not if t==Type::Bool||t==Type::Unknown=>Type::Bool,UnaryOp::Neg=>return Err(format!("cannot negate {}",t)),UnaryOp::Not=>return Err(format!("cannot apply ! to {}",t))}},
        Expr::Binary{left,op,right}=>{let a=self.expr(left)?.ty;let b=self.expr(right)?.ty;match op{BinaryOp::Add|BinaryOp::Sub|BinaryOp::Mul|BinaryOp::Div|BinaryOp::Mod=>if a.is_numeric()&&b.is_numeric(){Type::common_numeric(&a,&b).unwrap()}else if *op==BinaryOp::Add&&a==Type::String&&b==Type::String{Type::String}else if *op==BinaryOp::Add&&matches!((&a,&b),(Type::Array(_),Type::Array(_)))&&a.compatible_with(&b){a.clone()}else{return Err(format!("invalid operands: {} and {}",a,b))},BinaryOp::Eq|BinaryOp::Ne=>if a.compatible_with(&b){Type::Bool}else{return Err(format!("cannot compare {} and {}",a,b))},BinaryOp::Lt|BinaryOp::Le|BinaryOp::Gt|BinaryOp::Ge=>if a.is_numeric()&&b.is_numeric(){Type::Bool}else{return Err("ordering requires numeric operands".into())},BinaryOp::And|BinaryOp::Or=>if(a==Type::Bool||a==Type::Unknown)&&(b==Type::Bool||b==Type::Unknown){Type::Bool}else{return Err("logical operators require Bool operands".into())}},},
        Expr::Call{callee,args}=>{let name=match callee.as_ref(){Expr::Ident(n)=>n,_=>return Err("call target must currently be an identifier".into())};if name=="print"{for a in args{self.expr(a)?}Type::Unit}else{let(ps,r)=self.functions.get(name).cloned().ok_or_else(||format!("unknown function `{name}`"))?;if ps.len()!=args.len(){return Err(format!("function `{name}` expects {} arguments, got {}",ps.len(),args.len()))}for(i,a)in args.iter().enumerate(){let at=self.expr(a)?.ty;if ps[i]!=Type::Unknown&&!ps[i].compatible_with(&at){return Err(format!("argument {} to `{}` expects {}, found {}",i+1,name,ps[i],at))}}r}}
    };Ok(TypedExpr{expr:e.clone(),ty})}
}
