#[derive(Clone, Debug, PartialEq)]
pub struct Program { pub items: Vec<Item> }

#[derive(Clone, Debug, PartialEq)]
pub enum Item { Let { name: String, mutable: bool, value: Expr }, Fn { name: String, params: Vec<String>, body: Vec<Stmt> }, Stmt(Stmt) }

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt { Expr(Expr), Return(Option<Expr>), If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Vec<Stmt> }, While { condition: Expr, body: Vec<Stmt> } }

#[derive(Clone, Debug, PartialEq)]
pub enum Expr { Int(i64), Float(f64), Bool(bool), Str(String), Ident(String), Unary { op: UnaryOp, expr: Box<Expr> }, Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> }, Call { callee: Box<Expr>, args: Vec<Expr> } }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp { Neg, Not }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp { Add, Sub, Mul, Div, Mod, Eq, Ne, Lt, Le, Gt, Ge, And, Or }
