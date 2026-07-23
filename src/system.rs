use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;

use crate::object::{NativeData, Value};

pub fn make_system_module(root_dir: Rc<str>) -> IndexMap<String, Value> {
    let mut map = IndexMap::new();
    map.insert("os".to_string(), system_os());
    map.insert("exit".to_string(), system_exit());
    map.insert("args".to_string(), system_args());
    map.insert("getRoot".to_string(), system_get_root(root_dir));

    map
}

fn system_os() -> Value {
    Value::NativeFunction(NativeData::new(
        "os".to_string(),
        0,
        Rc::new(move |_| {
            #[cfg(target_os = "macos")]
            return Ok(Value::String("macos".to_string()));
            #[cfg(target_os = "linux")]
            return Ok(Value::String("linux".to_string()));
            #[cfg(target_os = "windows")]
            return Ok(Value::String("windows".to_string()));
        }),
    ))
}

fn system_exit() -> Value {
    Value::NativeFunction(NativeData::new(
        "exit".to_string(),
        1,
        Rc::new(move |args| {
            std::process::exit(match &args[0] {
                Value::Number(n) => *n as i32,
                other => return Err(format!("system.exit(): unsupported type '{}' expected type 'Number'", other).into())
            })
        })
    ))
}

fn system_args() -> Value {
    Value::NativeFunction(NativeData::new(
        "args".to_string(),
        0,
        Rc::new(move |_| {
            let args: Vec<Value> = std::env::args().skip(2).map(Value::String).collect();
            return Ok(Value::Array(Rc::new(RefCell::new(args))))
        })
    ))
}

fn system_get_root(root_dir: Rc<str>) -> Value {
    Value::NativeFunction(NativeData::new(
        "cwd".to_string(),
        0,
        Rc::new(move |_| {
            return Ok(Value::String(root_dir.to_string()))
        })
    ))
}