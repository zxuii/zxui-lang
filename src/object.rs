use std::{cell::RefCell, rc::Rc};

use crate::{ast::Stmt, environment::Environment};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Number(f64),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
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
