use std::rc::Rc;

use indexmap::IndexMap;

use crate::object::{NativeData, Value};

pub fn make_system_module() -> IndexMap<String, Value> {
    let mut map = IndexMap::new();
    map.insert("os".to_string(), system_os());

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
