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

thread_local! {
    static FMT_VISITED: RefCell<Vec<usize>> = RefCell::new(Vec::new());
}

impl Value {
    pub fn native_fun(
        name: String,
        arity: i32,
        fun: Rc<dyn Fn(Vec<Value>) -> Result<Value, String>>,
    ) -> Self {
        Self::NativeFunction(NativeData::new(name, arity, fun))
    }

    fn is_simple_scalar(&self) -> bool {
        matches!(
            self,
            Value::Null | Value::Number(_) | Value::Boolean(_) | Value::String(_)
        )
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
                let ptr = Rc::as_ptr(vec_ref) as usize;

                let already_visiting = FMT_VISITED.with(|v| v.borrow().contains(&ptr));
                if already_visiting {
                    return write!(f, "[...circular...]");
                }

                FMT_VISITED.with(|v| v.borrow_mut().push(ptr));
                let result = (|| {
                    let vec = vec_ref.borrow();
                    if vec.is_empty() {
                        return write!(f, "[]");
                    }

                    // heuristik: kalau semua elemen scalar sederhana dan totalnya pendek, satu baris
                    let all_simple = vec.iter().all(|v| v.is_simple_scalar());
                    if all_simple && vec.len() <= 8 {
                        write!(f, "[")?;
                        for (i, val) in vec.iter().enumerate() {
                            val.fmt(f, indent + 1)?;
                            if i < vec.len() - 1 {
                                write!(f, ", ")?;
                            }
                        }
                        return write!(f, "]");
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
                })();

                FMT_VISITED.with(|v| {
                    v.borrow_mut().pop();
                });
                result
            }

            Value::Map(map_ref) => {
                let ptr = Rc::as_ptr(map_ref) as usize;

                let already_visiting = FMT_VISITED.with(|v| v.borrow().contains(&ptr));
                if already_visiting {
                    return write!(f, "{{...circular...}}");
                }

                FMT_VISITED.with(|v| v.borrow_mut().push(ptr));
                let result = (|| {
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
                })();

                FMT_VISITED.with(|v| {
                    v.borrow_mut().pop();
                });
                result
            }

            Value::Function(fun) => write!(f, "[fun {}]", fun.name),
            Value::NativeFunction(fun) => write!(f, "[native fun {}]", fun.name),
            Value::Class(c) => write!(f, "[class {}]", c.name),
            Value::Instance(i) => write!(f, "[instance of {}]", i.class.name),
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

pub struct TypeRegistry {
    pub number: Rc<ClassData>,
    pub string: Rc<ClassData>,
    pub boolean: Rc<ClassData>,
    pub array: Rc<ClassData>,
    pub map: Rc<ClassData>,
    pub null: Rc<ClassData>,
    pub function: Rc<ClassData>,
    pub native_function: Rc<ClassData>,
    pub class: Rc<ClassData>,
}

impl TypeRegistry {
    pub fn class_for(&self, v: &Value) -> Rc<ClassData> {
        match v {
            Value::Number(_) => Rc::clone(&self.number),
            Value::String(_) => Rc::clone(&self.string),
            Value::Boolean(_) => Rc::clone(&self.boolean),
            Value::Array(_) => Rc::clone(&self.array),
            Value::Map(_) => Rc::clone(&self.map),
            Value::Null => Rc::clone(&self.null),
            Value::Function(_) => Rc::clone(&self.function),
            Value::NativeFunction(_) => Rc::clone(&self.native_function),
            Value::Class(_) => Rc::clone(&self.class),
            Value::Instance(inst) => Rc::clone(&inst.class),
        }
    }
}