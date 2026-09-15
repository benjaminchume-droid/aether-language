use crate::types::Type;
#[derive(Clone,Debug,PartialEq)] pub struct Program{pub items:Vec<Item>}
#[derive(Clone,Debug,PartialEq)] pub struct Param{pub name:String,pub ty:Option<Type>}
#[derive(Clone,Debug,PartialEq)] pub struct Field{pub name:String,pub ty:Type}
#[derive(Clone,Debug,PartialEq)] pub struct EnumVariant{pub name:String,pub payload:Vec<Type>}
#[derive(Clone,Debug,PartialEq)] pub enum Item{Import(String),Let{name:String,mutable:bool,annotation:Option<Type>,value:Expr},Fn{name:String,params:Vec<Param>,return_type:Option<Type>,body:Vec<Stmt>},Struct{name:String,fields:Vec<Field>},Enum{name:String,variants:Vec<EnumVariant>},Stmt(Stmt)}
#[derive(Clone,Debug,PartialEq)] pub struct MatchArm{pub pattern:Pattern,pub body:Vec<Stmt>}
#[derive(Clone,Debug,PartialEq)] pub enum Pattern{Wildcard,Enum{enum_name:String,variant:String,bindings:Vec<String>}}
#[derive(Clone,Debug,PartialEq)] pub enum Stmt{Expr(Expr),Let{name:String,mutable:bool,annotation:Option<Type>,value:Expr},Assign{target:Expr,value:Expr},Return(Option<Expr>),If{condition:Expr,then_branch:Vec<Stmt>,else_branch:Vec<Stmt>},While{condition:Expr,body:Vec<Stmt>},For{name:String,iterable:Expr,body:Vec<Stmt>},Loop{body:Vec<Stmt>},Match{value:Expr,arms:Vec<MatchArm>},Break,Continue}
#[derive(Clone,Debug,PartialEq)] pub enum Expr{Int(i64),Float(f64),Bool(bool),Str(String),Array(Vec<Expr>),Tuple(Vec<Expr>),Ident(String),StructInit{name:String,fields:Vec<(String,Expr)>},Member{target:Box<Expr>,field:String},EnumValue{enum_name:String,variant:String,payload:Vec<Expr>},Index{target:Box<Expr>,index:Box<Expr>},Unary{op:UnaryOp,expr:Box<Expr>},Binary{left:Box<Expr>,op:BinaryOp,right:Box<Expr>},Call{callee:Box<Expr>,args:Vec<Expr>}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum UnaryOp{Neg,Not}
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum BinaryOp{Add,Sub,Mul,Div,Mod,Eq,Ne,Lt,Le,Gt,Ge,And,Or}
