#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Expr>),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    BinOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    CompOp {
        left: Box<Expr>,
        op: CompOp,
        right: Box<Expr>,
    },
    LogicalOp {
        left: Box<Expr>,
        op: LogicalOp,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        line: usize,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
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
    EqEq,
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
    Let {
        name: String,
        expr: Expr,
    },
    Assign {
        name: String,
        expr: Expr,
    },
    Return(Expr),
    If {
        expr: Expr,
        block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
    While {
        expr: Expr,
        block: Vec<Stmt>,
    },
    ExprStmt(Expr),
    FunDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Break,
    Continue,
    Block(Vec<Stmt>),
}
