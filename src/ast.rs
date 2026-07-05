#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
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
    CompOp {
        left: Box<Expr>,
        op: CompOp,
        right: Box<Expr>
    },
    LogicalOp {
        left: Box<Expr>,
        op: LogicalOp,
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
    Not,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum CompOp {
    Lt,
    Gt,
    LtEq,
    GtEq,
    NotEq,
    EqEq
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