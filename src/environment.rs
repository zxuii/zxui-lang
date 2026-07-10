use crate::object::Value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosing(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Result<Value, String> {
        if let Some(val) = self.values.get(&name) {
            Ok(val.clone())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow().get(name)
            } else {
                Err(format!("Undefined variable '{}'.", name))
            }
        }
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), String> {
        if self.values.contains_key(&name) {
            self.values.insert(name.clone(), value);
            Ok(())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow_mut().assign(name, value)
            } else {
                Err(format!("Undefined variable '{}'.", name))
            }
        }
    }
}
