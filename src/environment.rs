use crate::object::Object;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
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

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Result<Object, String> {
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

    pub fn assign(&mut self, name: String, value: Object) -> Result<(), String> {
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