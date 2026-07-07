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

impl Value {
    pub fn native_fun(
        name: String,
        arity: i32,
        fun: Rc<dyn Fn(Vec<Value>) -> Result<Value, String>>,
    ) -> Self {
        Self::NativeFunction { name, arity, fun }
    }

    fn fmt(&self, f: &mut std::fmt::Formatter, indent: usize) -> std::fmt::Result {
        let spaces = "  ".repeat(indent);
        let next_spaces = "  ".repeat(indent + 1);

        match self {
            Value::Null => write!(f, "null"),
            Value::Number(num) => write!(f, "{}", num),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(str) => write!(f, "\"{}\"", str),
            Value::Array(vec_ref) => {
                let vec = vec_ref.borrow();
                if vec.is_empty() {
                    return write!(f, "[]");
                }
                writeln!(f, "[")?;
                for (i, val) in vec.iter().enumerate() {
                    write!(f, "{}", next_spaces)?;
                    val.fmt(f, indent + 1)?;
                    if i < vec.len() - 1 {
                        writeln!(f, ",")?;
                    } else {
                        writeln!(f)?;
                    }
                }
                write!(f, "{}]", spaces)
            }
            Value::Map(map_ref) => {
                let map = map_ref.borrow();
                if map.is_empty() {
                    return write!(f, "{{}}");
                }
                writeln!(f, "{{")?;
                for (i, (key, val)) in map.iter().enumerate() {
                    write!(f, "{}{} = ", next_spaces, key)?;
                    val.fmt(f, indent + 1)?;
                    if i < map.len() - 1 {
                        writeln!(f, ",")?;
                    } else {
                        writeln!(f)?;
                    }
                }
                write!(f, "{}}}", spaces)
            }
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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt(f, 0)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
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