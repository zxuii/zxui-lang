use indexmap::IndexMap;
use std::{any::Any, cell::RefCell, rc::Rc};

use crate::{ast::Stmt, environment::Environment};

#[derive(Clone)]
pub struct FunData {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
}

impl FunData {
    pub fn new(name: String, params: Vec<String>, body: Vec<Stmt>, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            name, params, body, closure,
        }
    }
}

#[derive(Clone)]
pub struct NativeData {
    pub name: String,
    pub arity: i32,
    pub fun: Rc<dyn Fn(Vec<Value>) -> Result<Value, String>>,
}

impl NativeData {
    pub fn new(name: String, arity: i32, fun: Rc<dyn Fn(Vec<Value>) -> Result<Value, String>>) -> Self {
        Self {
            name, arity, fun
        }
    }
}

#[derive(Clone)]
pub enum MethodKind {
    User(Rc<FunData>),
    Native(Rc<NativeMethodData>),
}

pub struct NativeMethodData {
    pub name: String,
    pub arity: i32,
    pub fun: Box<dyn Fn(Value, Vec<Value>) -> Result<Value, String>>,
}

#[derive(Clone)]
pub struct ClassData {
    pub name: String,
    pub methods: IndexMap<String, MethodKind>,
    pub static_methods: IndexMap<String, MethodKind>,
    pub superclass: Option<Rc<ClassData>>,
    pub native_get: Option<Rc<dyn Fn(&InstanceData, &str) -> Option<Value>>>,
    pub native_set: Option<Rc<dyn Fn(&InstanceData, &str, Value) -> Result<(), String>>>,
}

impl ClassData {
    pub fn new(
        name: String,
        methods: IndexMap<String, MethodKind>,
        static_methods: IndexMap<String, MethodKind>,
        superclass: Option<Rc<ClassData>>,
    ) -> Self {
        Self {
            name,
            methods,
            static_methods,
            superclass,
            native_get: None,
            native_set: None,
        }
    }

    pub fn new_native(
        name: String,
        methods: IndexMap<String, MethodKind>,
        static_methods: IndexMap<String, MethodKind>,
        native_get: Option<Rc<dyn Fn(&InstanceData, &str) -> Option<Value>>>,
        native_set: Option<Rc<dyn Fn(&InstanceData, &str, Value) -> Result<(), String>>>,
    ) -> Self {
        Self {
            name,
            methods,
            static_methods,
            superclass: None,
            native_get,
            native_set,
        }
    }
}

pub struct InstanceData {
    pub class: Rc<ClassData>,
    pub fields: RefCell<IndexMap<String, Value>>,
    pub native: RefCell<Option<Rc<dyn Any>>>,
}

#[derive(Clone)]
pub enum Value {
    Null,
    Number(f64),
    Boolean(bool),
    String(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<IndexMap<String, Value>>>),
    Function(FunData),
    NativeFunction(NativeData),
    Class(Rc<ClassData>),
    Instance(Rc<InstanceData>),
}

impl Value {
    pub fn native_fun(
        name: String,
        arity: i32,
        fun: Rc<dyn Fn(Vec<Value>) -> Result<Value, String>>,
    ) -> Self {
        Self::NativeFunction(NativeData::new(name, arity, fun))
    }

    fn fmt(&self, f: &mut std::fmt::Formatter, indent: usize) -> std::fmt::Result {
        let spaces = "  ".repeat(indent);
        let next_spaces = "  ".repeat(indent + 1);

        match self {
            Value::Null => write!(f, "null"),
            Value::Number(num) => write!(f, "{}", num),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(str) => {
                if indent == 0 {
                    write!(f, "{}", str)
                } else {
                    write!(f, "\"{}\"", str)
                }
            }
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
            Value::Function(fun) => write!(f, "[fun {}]", fun.name),
            Value::NativeFunction(fun) => write!(f, "[native fun {}]", fun.name),
            Value::Class(c) => write!(f, "[class {}]", c.name),
            Value::Instance(i) => write!(f, "[instance of {}", i.class.name),
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
