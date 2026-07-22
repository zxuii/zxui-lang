use std::rc::Rc;

use indexmap::IndexMap;

use crate::object::{NativeData, Value};

pub fn make_system_module() -> IndexMap<String, Value> {
    let mut map = IndexMap::new();
    map.insert("os".to_string(), system_os());
    map.insert("exit".to_string(), system_exit());

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