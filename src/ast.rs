#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    Null,
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    BinOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    NoOp,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Program(Vec<Stmt>),
    Let { name: String, expr: Expr },
    Assign { name: String, expr: Expr },
    Return(Expr),
    ExprStmt(Expr),
    FunDecl { name: String, params: Vec<String>, body: Vec<Stmt> },
    Block(Vec<Stmt>),
}