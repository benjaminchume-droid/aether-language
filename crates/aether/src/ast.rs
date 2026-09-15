#[derive(Debug, Clone, PartialEq)]
pub struct Program { pub items: Vec<Item> }

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Let { name: String, mutable: bool, value: Expr },
    Fn { name: String, params: Vec<String>, body: Vec<Stmt> },
    Stmt(Stmt),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Let { name: String, mutable: bool, value: Expr },
    Assign { target: Expr, value: Expr },
    Return(Option<Expr>),
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Vec<Stmt> },
    While { condition: Expr, body: Vec<Stmt> },
    For { name: String, iterable: Expr, body: Vec<Stmt> },
    Loop { body: Vec<Stmt> },
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64), Float(f64), Bool(bool), Str(String), Array(Vec<Expr>), Ident(String),
    Index { target: Box<Expr>, index: Box<Expr> },
    Unary { op: UnaryOp, expr: Box<Expr> },
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp { Neg, Not }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp { Add, Sub, Mul, Div, Mod, Eq, Ne, Lt, Le, Gt, Ge, And, Or }
