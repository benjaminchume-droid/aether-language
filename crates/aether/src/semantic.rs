use std::collections::HashMap;

use crate::ast::*;
use crate::types::Type;

#[derive(Clone, Debug)]
pub struct Symbol { pub ty: Type, pub mutable: bool }

#[derive(Default)]
pub struct Analyzer { scopes: Vec<HashMap<String, Symbol>>, functions: HashMap<String, (Vec<Type>, Type)> }

impl Analyzer {
    pub fn analyze(program: &Program) -> Result<(), Vec<String>> {
        let mut a = Self::default();
        a.push_scope();
        let mut errors = Vec::new();
        for item in &program.items {
            if let Err(e) = a.item(item) { errors.extend(e); }
        }
        a.pop_scope();
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    fn push_scope(&mut self) { self.scopes.push(HashMap::new()); }
    fn pop_scope(&mut self) { self.scopes.pop(); }

    fn define(&mut self, name: &str, sym: Symbol) -> Result<(), String> {
        let scope = self.scopes.last_mut().unwrap();
        if scope.contains_key(name) { return Err(format!("duplicate definition of `{name}`")); }
        scope.insert(name.to_owned(), sym); Ok(())
    }

    fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.scopes.iter().rev().find_map(|s| s.get(name))
    }

    fn item(&mut self, item: &Item) -> Result<(), Vec<String>> {
        match item {
            Item::Let { name, mutable, value } => match self.expr(value) {
                Ok(ty) => self.define(name, Symbol { ty, mutable: *mutable }).map_err(|e| vec![e]),
                Err(e) => Err(vec![e]),
            },
            Item::Fn { name, params, body } => {
                let sig = (vec![Type::Unknown; params.len()], Type::Unknown);
                if let Err(e) = self.define(name, Symbol { ty: Type::Unknown, mutable: false }) { return Err(vec![e]); }
                self.functions.insert(name.clone(), sig);
                self.push_scope();
                for p in params { if let Err(e) = self.define(p, Symbol { ty: Type::Unknown, mutable: false }) { self.pop_scope(); return Err(vec![e]); } }
                let mut errors = Vec::new();
                for s in body { if let Err(e) = self.stmt(s) { errors.extend(e); } }
                self.pop_scope();
                if errors.is_empty() { Ok(()) } else { Err(errors) }
            }
            Item::Stmt(s) => self.stmt(s),
        }
    }

    fn stmt(&mut self, stmt: &Stmt) -> Result<(), Vec<String>> {
        match stmt {
            Stmt::Expr(e) => self.expr(e).map(|_|()).map_err(|e| vec![e]),
            Stmt::Return(e) => match e { Some(x) => self.expr(x).map(|_|()).map_err(|e| vec![e]), None => Ok(()) },
            Stmt::If { condition, then_branch, else_branch } => {
                let mut errors = Vec::new();
                match self.expr(condition) { Ok(Type::Bool) | Ok(Type::Unknown) => {}, Ok(t) => errors.push(format!("if condition must be Bool, found {t}")), Err(e) => errors.push(e) }
                self.push_scope(); for s in then_branch { if let Err(es)=self.stmt(s){errors.extend(es)} } self.pop_scope();
                self.push_scope(); for s in else_branch { if let Err(es)=self.stmt(s){errors.extend(es)} } self.pop_scope();
                if errors.is_empty(){Ok(())}else{Err(errors)}
            }
            Stmt::While { condition, body } => {
                let mut errors=Vec::new();
                match self.expr(condition){Ok(Type::Bool)|Ok(Type::Unknown)=>{},Ok(t)=>errors.push(format!("while condition must be Bool, found {t}")),Err(e)=>errors.push(e)}
                self.push_scope();for s in body{if let Err(es)=self.stmt(s){errors.extend(es)}}self.pop_scope();
                if errors.is_empty(){Ok(())}else{Err(errors)}
            }
        }
    }

    fn expr(&mut self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Int(_) => Ok(Type::Int), Expr::Float(_) => Ok(Type::Float), Expr::Bool(_) => Ok(Type::Bool), Expr::Str(_) => Ok(Type::String),
            Expr::Ident(name) => self.lookup(name).map(|s| s.ty.clone()).ok_or_else(|| format!("unknown identifier `{name}`")),
            Expr::Unary { op, expr } => { let t=self.expr(expr)?; match op { UnaryOp::Neg if t.is_numeric()=>Ok(t), UnaryOp::Not if t==Type::Bool||t==Type::Unknown=>Ok(Type::Bool), UnaryOp::Neg=>Err(format!("cannot negate {t}")), UnaryOp::Not=>Err(format!("cannot apply ! to {t}")) } }
            Expr::Binary { left, op, right } => { let a=self.expr(left)?; let b=self.expr(right)?; match op {
                BinaryOp::Add|BinaryOp::Sub|BinaryOp::Mul|BinaryOp::Div|BinaryOp::Mod => { if a.is_numeric()&&b.is_numeric(){Ok(Type::common_numeric(&a,&b).unwrap())} else if *op==BinaryOp::Add&&(a==Type::String&&b==Type::String){Ok(Type::String)} else {Err(format!("invalid operands: {a} and {b}"))} },
                BinaryOp::Eq|BinaryOp::Ne => { if a.compatible_with(&b){Ok(Type::Bool)}else{Err(format!("cannot compare {a} and {b}"))} },
                BinaryOp::Lt|BinaryOp::Le|BinaryOp::Gt|BinaryOp::Ge => { if a.is_numeric()&&b.is_numeric(){Ok(Type::Bool)}else{Err(format!("ordering requires numeric operands, found {a} and {b}"))} },
                BinaryOp::And|BinaryOp::Or => { if (a==Type::Bool||a==Type::Unknown)&&(b==Type::Bool||b==Type::Unknown){Ok(Type::Bool)}else{Err("logical operators require Bool operands".into())} }
            }}
            Expr::Call { callee, args } => { let name=match callee.as_ref(){Expr::Ident(n)=>n, _=>return Err("call target must currently be an identifier".into())}; if let Some((params,ret))=self.functions.get(name){if params.len()!=args.len(){return Err(format!("function `{name}` expects {} arguments, got {}",params.len(),args.len()));}for arg in args{self.expr(arg)?;}Ok(ret.clone())}else{Err(format!("unknown function `{name}`"))} }
        }
    }
}
