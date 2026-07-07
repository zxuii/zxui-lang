use std::{cell::RefCell, rc::Rc};
use indexmap::IndexMap;

use crate::{ast::Stmt, environment::Environment};

#[derive(Clone)]
pub enum Value {
    Null,
    Number(f64),
    Boolean(bool),
    String(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<IndexMap<String, Value>>>),
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
    NativeFunction {
        name: String,
        arity: i32,
        fun: Rc<dyn Fn(Vec<Value>) -> Result<Value, String>>,
    },
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Number(num) => write!(f, "{}", num),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(str) => write!(f, "{}", str),
            Value::Array(vec) => write!(f, "{:?}", vec.borrow()),
            Value::Map(vec) => write!(f, "{:?}", vec.borrow()),
            Value::Function {
                name,
                params: _,
                body: _,
                closure: _,
            } => write!(f, "[fun {name}]"),
            Value::NativeFunction { name, .. } => write!(f, "[native fun {name}]"),
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Number(num) => write!(f, "{}", num),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(str) => write!(f, "{}", str),
            Value::Array(vec) => write!(f, "{:?}", vec),
            Value::Map(vec) => write!(f, "{:?}", vec.borrow()),
            Value::Function {
                name,
                params: _,
                body: _,
                closure: _,
            } => write!(f, "[fun {name}]"),
            Value::NativeFunction { name, .. } => write!(f, "[native fun {name}]"),
        }
    }
}

impl Value {
    pub fn native_fun(
        name: String,
        arity: i32,
        fun: Rc<dyn Fn(Vec<Value>) -> Result<Value, String>>,
    ) -> Self {
        Self::NativeFunction { name, arity, fun }
    }
}

// impl Value {
//     fn equals(&self, other: &Value) -> bool {
//         match (self, other) {
//             (Value::Null, Value::Null) => true,
//             (_, Value::Null) => false,
//             (Value::Null, _) => false,
//             (Value::Number(left), Value::Number(right)) => left == right,
//         }
//     }
// }
