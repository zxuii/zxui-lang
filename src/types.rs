use crate::object::{ClassData, MethodKind, NativeMethodData, TypeRegistry, Value};

use indexmap::IndexMap;
use std::cell::RefCell;
use std::rc::Rc;

fn method(
    name: &str,
    arity: i32,
    fun: impl Fn(Value, Vec<Value>) -> Result<Value, String> + 'static,
) -> (String, MethodKind) {
    (
        name.to_string(),
        MethodKind::Native(Rc::new(NativeMethodData {
            name: name.to_string(),
            arity,
            fun: Box::new(fun),
        })),
    )
}

fn expect_self_string(v: &Value, fname: &str) -> Result<String, String> {
    match v {
        Value::String(s) => Ok(s.clone()),
        other => Err(format!("{}(): called on non-string value '{}'", fname, other)),
    }
}

fn expect_self_array(v: &Value, fname: &str) -> Result<Rc<RefCell<Vec<Value>>>, String> {
    match v {
        Value::Array(a) => Ok(Rc::clone(a)),
        other => Err(format!("{}(): called on non-array value '{}'", fname, other)),
    }
}

fn expect_self_map(v: &Value, fname: &str) -> Result<Rc<RefCell<IndexMap<String, Value>>>, String> {
    match v {
        Value::Map(m) => Ok(Rc::clone(m)),
        other => Err(format!("{}(): called on non-map value '{}'", fname, other)),
    }
}

fn expect_self_number(v: &Value, fname: &str) -> Result<f64, String> {
    match v {
        Value::Number(n) => Ok(*n),
        other => Err(format!("{}(): called on non-number value '{}'", fname, other)),
    }
}

fn build_number_class() -> Rc<ClassData> {
    let mut methods = IndexMap::new();
    methods.extend([
        method("floor", 0, |self_val, _| {
            Ok(Value::Number(expect_self_number(&self_val, "floor")?.floor()))
        }),
        method("ceil", 0, |self_val, _| {
            Ok(Value::Number(expect_self_number(&self_val, "ceil")?.ceil()))
        }),
        method("round", 0, |self_val, _| {
            Ok(Value::Number(expect_self_number(&self_val, "round")?.round()))
        }),
        method("abs", 0, |self_val, _| {
            Ok(Value::Number(expect_self_number(&self_val, "abs")?.abs()))
        }),
    ]);
    Rc::new(ClassData::new("Number".to_string(), methods, IndexMap::new(), None))
}

fn build_string_class() -> Rc<ClassData> {
    let mut methods = IndexMap::new();
    methods.extend([
        method("len", 0, |self_val, _| {
            let s = expect_self_string(&self_val, "len")?;
            Ok(Value::Number(s.chars().count() as f64))
        }),
        method("upper", 0, |self_val, _| {
            Ok(Value::String(expect_self_string(&self_val, "upper")?.to_uppercase()))
        }),
        method("lower", 0, |self_val, _| {
            Ok(Value::String(expect_self_string(&self_val, "lower")?.to_lowercase()))
        }),
        method("trim", 0, |self_val, _| {
            Ok(Value::String(expect_self_string(&self_val, "trim")?.trim().to_string()))
        }),
        method("split", 1, |self_val, args| {
            let s = expect_self_string(&self_val, "split")?;
            let sep = match &args[0] {
                Value::String(sep) => sep.clone(),
                other => return Err(format!("split(): argument must be a string, got '{}'", other)),
            };
            let parts: Vec<Value> = if sep.is_empty() {
                s.chars().map(|c| Value::String(c.to_string())).collect()
            } else {
                s.split(sep.as_str()).map(|p| Value::String(p.to_string())).collect()
            };
            Ok(Value::Array(Rc::new(RefCell::new(parts))))
        }),
        method("contains", 1, |self_val, args| {
            let s = expect_self_string(&self_val, "contains")?;
            let needle = match &args[0] {
                Value::String(n) => n.clone(),
                other => return Err(format!("contains(): argument must be a string, got '{}'", other)),
            };
            Ok(Value::Boolean(s.contains(needle.as_str())))
        }),
        method("replace", 2, |self_val, args| {
            let s = expect_self_string(&self_val, "replace")?;
            let from = match &args[0] {
                Value::String(f) => f.clone(),
                other => return Err(format!("replace(): first argument must be a string, got '{}'", other)),
            };
            let to = match &args[1] {
                Value::String(t) => t.clone(),
                other => return Err(format!("replace(): second argument must be a string, got '{}'", other)),
            };
            Ok(Value::String(s.replace(from.as_str(), to.as_str())))
        }),
    ]);
    Rc::new(ClassData::new("String".to_string(), methods, IndexMap::new(), None))
}

fn build_array_class() -> Rc<ClassData> {
    let mut methods = IndexMap::new();
    methods.extend([
        method("len", 0, |self_val, _| {
            let arr = expect_self_array(&self_val, "len")?;
            Ok(Value::Number(arr.borrow().len() as f64))
        }),
        method("push", 1, |self_val, args| {
            let arr = expect_self_array(&self_val, "push")?;
            arr.borrow_mut().push(args[0].clone());
            Ok(Value::Null)
        }),
        method("pop", 0, |self_val, _| {
            let arr = expect_self_array(&self_val, "pop")?;
            arr.borrow_mut().pop().ok_or_else(|| "pop(): cannot pop empty array.".to_string())
        }),
        method("remove", 1, |self_val, args| {
            let arr = expect_self_array(&self_val, "remove")?;
            let idx = match &args[0] {
                Value::Number(n) if *n >= 0.0 => *n as usize,
                Value::Number(_) => return Err("remove(): index cannot be negative.".to_string()),
                other => return Err(format!("remove(): argument must be a number, got '{}'", other)),
            };
            let len = arr.borrow().len();
            if idx >= len {
                return Err(format!(
                    "remove(): index out of bounds. need index of {}, but only has {} indices.",
                    idx, len
                ));
            }
            Ok(arr.borrow_mut().remove(idx))
        }),
        method("clear", 0, |self_val, _| {
            let arr = expect_self_array(&self_val, "clear")?;
            arr.borrow_mut().clear();
            Ok(Value::Null)
        }),
    ]);
    Rc::new(ClassData::new("Array".to_string(), methods, IndexMap::new(), None))
}

fn build_map_class() -> Rc<ClassData> {
    let mut methods = IndexMap::new();
    methods.extend([
        method("len", 0, |self_val, _| {
            let m = expect_self_map(&self_val, "len")?;
            Ok(Value::Number(m.borrow().len() as f64))
        }),
        method("keys", 0, |self_val, _| {
            let m = expect_self_map(&self_val, "keys")?;
            let keys: Vec<Value> = m.borrow().keys().map(|k| Value::String(k.clone())).collect();
            Ok(Value::Array(Rc::new(RefCell::new(keys))))
        }),
        method("values", 0, |self_val, _| {
            let m = expect_self_map(&self_val, "values")?;
            let values: Vec<Value> = m.borrow().values().cloned().collect();
            Ok(Value::Array(Rc::new(RefCell::new(values))))
        }),
        method("hasKey", 1, |self_val, args| {
            let m = expect_self_map(&self_val, "hasKey")?;
            let key = match &args[0] {
                Value::String(k) => k.clone(),
                other => return Err(format!("hasKey(): argument must be a string, got '{}'", other)),
            };
            Ok(Value::Boolean(m.borrow().contains_key(&key)))
        }),
        method("remove", 1, |self_val, args| {
            let m = expect_self_map(&self_val, "remove")?;
            let key = match &args[0] {
                Value::String(k) => k.clone(),
                other => return Err(format!("remove(): argument must be a string, got '{}'", other)),
            };
            Ok(m.borrow_mut().shift_remove(&key).unwrap_or(Value::Null))
        }),
        method("clear", 0, |self_val, _| {
            let m = expect_self_map(&self_val, "clear")?;
            m.borrow_mut().clear();
            Ok(Value::Null)
        }),
    ]);
    Rc::new(ClassData::new("Map".to_string(), methods, IndexMap::new(), None))
}

fn build_boolean_class() -> Rc<ClassData> {
    Rc::new(ClassData::new("Boolean".to_string(), IndexMap::new(), IndexMap::new(), None))
}
fn build_null_class() -> Rc<ClassData> {
    Rc::new(ClassData::new("Null".to_string(), IndexMap::new(), IndexMap::new(), None))
}
fn build_function_class() -> Rc<ClassData> {
    Rc::new(ClassData::new("Function".to_string(), IndexMap::new(), IndexMap::new(), None))
}
fn build_native_function_class() -> Rc<ClassData> {
    Rc::new(ClassData::new("NativeFunction".to_string(), IndexMap::new(), IndexMap::new(), None))
}
fn build_class_class() -> Rc<ClassData> {
    Rc::new(ClassData::new("Class".to_string(), IndexMap::new(), IndexMap::new(), None))
}

pub fn build_type_registry() -> TypeRegistry {
    TypeRegistry {
        number: build_number_class(),
        string: build_string_class(),
        boolean: build_boolean_class(),
        array: build_array_class(),
        map: build_map_class(),
        null: build_null_class(),
        function: build_function_class(),
        native_function: build_native_function_class(),
        class: build_class_class(),
    }
}

pub fn convert_to_number(v: &Value) -> Result<Value, String> {
    match v {
        Value::String(s) => s
            .trim()
            .parse::<f64>()
            .map(Value::Number)
            .map_err(|e| format!("Number(): failed to parse '{}': {}", s, e)),
        Value::Boolean(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
        Value::Number(n) => Ok(Value::Number(*n)),
        other => Err(format!("Number(): type '{}' cannot be converted to number.", other)),
    }
}

pub fn convert_to_boolean(v: &Value) -> Value {
    match v {
        Value::Boolean(b) => Value::Boolean(*b),
        Value::Null => Value::Boolean(false),
        Value::Number(n) => Value::Boolean(*n != 0.0),
        Value::String(s) => Value::Boolean(!s.is_empty()),
        Value::Array(a) => Value::Boolean(!a.borrow().is_empty()),
        Value::Map(m) => Value::Boolean(!m.borrow().is_empty()),
        _ => Value::Boolean(true),
    }
}

pub fn convert_to_array(v: &Value) -> Result<Value, String> {
    match v {
        Value::Array(a) => Ok(Value::Array(Rc::clone(a))),
        Value::String(s) => {
            let chars: Vec<Value> = s.chars().map(|c| Value::String(c.to_string())).collect();
            Ok(Value::Array(Rc::new(RefCell::new(chars))))
        }
        Value::Map(m) => {
            let pairs: Vec<Value> = m
                .borrow()
                .iter()
                .map(|(k, v)| {
                    let mut pair = IndexMap::new();
                    pair.insert("key".to_string(), Value::String(k.clone()));
                    pair.insert("val".to_string(), v.clone());
                    Value::Map(Rc::new(RefCell::new(pair)))
                })
                .collect();
            Ok(Value::Array(Rc::new(RefCell::new(pairs))))
        }
        other => Err(format!("Array(): type '{}' cannot be converted to array.", other)),
    }
}