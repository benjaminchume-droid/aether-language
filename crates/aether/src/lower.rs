use crate::ast;
use crate::hir;

pub fn lower(program: &ast::Program) -> hir::Program {
    hir::Program { items: program.items.iter().map(item).collect() }
}

fn item(i: &ast::Item) -> hir::Item {
    match i {
        ast::Item::Let { name, mutable, annotation, value } => hir::Item::Let {
            name: name.clone(), mutable: *mutable, annotation: annotation.clone(), value: expr(value)
        },
        ast::Item::Fn { name, params, return_type, body } => hir::Item::Fn {
            name: name.clone(),
            params: params.iter().map(|p| hir::Param { name: p.name.clone(), ty: p.ty.clone() }).collect(),
            return_type: return_type.clone(),
            body: body.iter().map(stmt).collect()
        },
        ast::Item::Struct { name, fields } => hir::Item::Struct {
            name: name.clone(), fields: fields.iter().map(|f| hir::Field { name: f.name.clone(), ty: f.ty.clone() }).collect()
        },
        ast::Item::Stmt(s) => hir::Item::Stmt(stmt(s)),
    }
}

fn stmt(s: &ast::Stmt) -> hir::Stmt {
    match s {
        ast::Stmt::Expr(e) => hir::Stmt::Expr(expr(e)),
        ast::Stmt::Let { name, mutable, annotation, value } => hir::Stmt::Let {
            name: name.clone(), mutable: *mutable, annotation: annotation.clone(), value: expr(value)
        },
        ast::Stmt::Assign { target, value } => hir::Stmt::Assign { target: expr(target), value: expr(value) },
        ast::Stmt::Return(e) => hir::Stmt::Return(e.as_ref().map(expr)),
        ast::Stmt::If { condition, then_branch, else_branch } => hir::Stmt::If {
            condition: expr(condition), then_branch: then_branch.iter().map(stmt).collect(), else_branch: else_branch.iter().map(stmt).collect()
        },
        ast::Stmt::While { condition, body } => hir::Stmt::While { condition: expr(condition), body: body.iter().map(stmt).collect() },
        ast::Stmt::For { name, iterable, body } => hir::Stmt::For { name: name.clone(), iterable: expr(iterable), body: body.iter().map(stmt).collect() },
        ast::Stmt::Loop { body } => hir::Stmt::Loop { body: body.iter().map(stmt).collect() },
        ast::Stmt::Break => hir::Stmt::Break,
        ast::Stmt::Continue => hir::Stmt::Continue,
    }
}

fn expr(e: &ast::Expr) -> hir::Expr {
    match e {
        ast::Expr::Int(v) => hir::Expr::Int(*v),
        ast::Expr::Float(v) => hir::Expr::Float(*v),
        ast::Expr::Bool(v) => hir::Expr::Bool(*v),
        ast::Expr::Str(v) => hir::Expr::Str(v.clone()),
        ast::Expr::Array(values) => hir::Expr::Array(values.iter().map(expr).collect()),
        ast::Expr::Ident(v) => hir::Expr::Ident(v.clone()),
        ast::Expr::StructInit { name, fields } => hir::Expr::StructInit { name: name.clone(), fields: fields.iter().map(|(n, v)| (n.clone(), expr(v))).collect() },
        ast::Expr::Member { target, field } => hir::Expr::Member { target: Box::new(expr(target)), field: field.clone() },
        ast::Expr::Index { target, index } => hir::Expr::Index { target: Box::new(expr(target)), index: Box::new(expr(index)) },
        ast::Expr::Unary { op, expr: inner } => hir::Expr::Unary {
            op: match op { ast::UnaryOp::Neg => hir::UnaryOp::Neg, ast::UnaryOp::Not => hir::UnaryOp::Not }, expr: Box::new(expr(inner))
        },
        ast::Expr::Binary { left, op, right } => hir::Expr::Binary {
            left: Box::new(expr(left)),
            op: match op {
                ast::BinaryOp::Add => hir::BinaryOp::Add, ast::BinaryOp::Sub => hir::BinaryOp::Sub,
                ast::BinaryOp::Mul => hir::BinaryOp::Mul, ast::BinaryOp::Div => hir::BinaryOp::Div,
                ast::BinaryOp::Mod => hir::BinaryOp::Mod, ast::BinaryOp::Eq => hir::BinaryOp::Eq,
                ast::BinaryOp::Ne => hir::BinaryOp::Ne, ast::BinaryOp::Lt => hir::BinaryOp::Lt,
                ast::BinaryOp::Le => hir::BinaryOp::Le, ast::BinaryOp::Gt => hir::BinaryOp::Gt,
                ast::BinaryOp::Ge => hir::BinaryOp::Ge, ast::BinaryOp::And => hir::BinaryOp::And,
                ast::BinaryOp::Or => hir::BinaryOp::Or,
            },
            right: Box::new(expr(right))
        },
        ast::Expr::Call { callee, args } => hir::Expr::Call { callee: Box::new(expr(callee)), args: args.iter().map(expr).collect() },
    }
}
