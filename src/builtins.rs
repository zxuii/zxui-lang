use crate::object::Value;

use libloading::{Library, Symbol};
use std::{
    cell::RefCell,
    ffi::CString,
    io::{self, Write},
    rc::Rc,
};

// helper permudah hidup
fn expect_number(v: &Value, fname: &str, i: usize) -> Result<f64, String> {
    match v {
        Value::Number(n) => Ok(*n),
        other => Err(format!(
            "{}(): argument {} must be a number, got '{}'.",
            fname,
            i + 1,
            other
        )),
    }
}

fn expect_string(v: &Value, fname: &str, i: usize) -> Result<String, String> {
    match v {
        Value::String(s) => Ok(s.clone()),
        other => Err(format!(
            "{}(): argument {} must be a string, got '{}'.",
            fname,
            i + 1,
            other
        )),
    }
}

// fn expect_boolean(v: &Value, fname: &str, i: usize) -> Result<bool, String> {
//     match v {
//         Value::Boolean(b) => Ok(*b),
//         other => Err(format!("{}(): argument {} must be a boolean, got '{}'.", fname, i + 1, other)),
//     }
// }

// -------------------------- UNTUK RAYLIB --------------------------

// fungsi fungsi raylib
type InitWindowFn = unsafe extern "C" fn(width: i32, height: i32, title: *const i8);
type WindowShouldCloseFn = unsafe extern "C" fn() -> bool;

// untuk mempermudah buat struct
pub struct Raylib {
    _lib: Library,
    pub init_window: InitWindowFn,
    pub window_should_close: WindowShouldCloseFn,
}

impl Raylib {
    pub fn new(lib_path: String) -> Result<Self, libloading::Error> {
        unsafe {
            let lib = Library::new(lib_path)?;
            let init_window = {
                let sym: Symbol<InitWindowFn> = lib.get(b"InitWindow\0")?;
                *sym
            };
            let window_should_close = {
                let sym: Symbol<WindowShouldCloseFn> = lib.get(b"WindowShouldClose\0")?;
                *sym
            };
            Ok(Self {
                _lib: lib,
                init_window,
                window_should_close,
            })
        }
    }
}

pub fn raylib_init_window(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "initWindow".to_string(),
        3,
        Rc::new(move |args| -> Result<Value, String> {
            let width = expect_number(&args[0], "initWindow", 0)? as i32;
            let height = expect_number(&args[1], "initWindow", 1)? as i32;
            let title = expect_string(&args[2], "initWindow", 2)?;
            let title_c = CString::new(title).unwrap();

            unsafe { (raylib.init_window)(width, height, title_c.as_ptr()) };
            Ok(Value::Null)
        }),
    )
}

pub fn raylib_windows_should_close(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "windowShouldClose".to_string(),
        0,
        Rc::new(move |_| -> Result<Value, String> {
            let val = unsafe { (raylib.window_should_close)() };
            Ok(Value::Boolean(val))
        }),
    )
}

// -------------------- UNTUK NATIVE BIASA --------------------------

pub fn native_println(args: Vec<Value>) -> Result<Value, String> {
    let _ = native_print(args);
    println!();
    Ok(Value::Null)
}

pub fn native_print(args: Vec<Value>) -> Result<Value, String> {
    for arg in &args {
        print!("{}", arg);
    }
    Ok(Value::Null)
}

pub fn native_readline(args: Vec<Value>) -> Result<Value, String> {
    let mut input = String::new();

    print!("{}", args[0]);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");

    let trimmed = input.trim_end_matches(['\n', '\r']).to_string();

    Ok(Value::String(trimmed))
}

pub fn native_typeof(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::String(_) => Ok(Value::String("string".to_string())),
        Value::Array(_) => Ok(Value::String("array".to_string())),
        Value::Map(_) => Ok(Value::String("map".to_string())),
        Value::Number(_) => Ok(Value::String("number".to_string())),
        Value::Function {
            name: _,
            body: _,
            closure: _,
            params: _,
        } => Ok(Value::String("fun".to_string())),
        Value::Boolean(_) => Ok(Value::String("boolean".to_string())),
        Value::Null => Ok(Value::String("null".to_string())),
        Value::NativeFunction {
            name: _,
            arity: _,
            fun: _,
        } => Ok(Value::String("native fun".to_string())),
    }
}

pub fn native_number(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::String(str) => match str.parse::<f64>() {
            Ok(n) => Ok(Value::Number(n)),
            Err(e) => Err(format!(
                "number(): failed to parse number from string '{}': {}",
                str,
                e.to_string()
            )),
        },

        Value::Boolean(b) => {
            if *b {
                Ok(Value::Number(1.0))
            } else {
                Ok(Value::Number(0.0))
            }
        }

        Value::Number(num) => Ok(Value::Number(*num)),

        _ => Err(format!(
            "number(): type '{}' cannot be converted to number.",
            args[0]
        )),
    }
}

pub fn native_string(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::String(format!("{}", args[0])))
}

pub fn native_boolean(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Boolean(b) => Ok(Value::Boolean(*b)),
        Value::Null => Ok(Value::Boolean(false)),
        Value::Number(num) => Ok(Value::Boolean(*num != 0.0)),
        Value::String(str) => Ok(Value::Boolean(!str.is_empty())),
        Value::Array(arr) => Ok(Value::Boolean(!arr.borrow().is_empty())),
        Value::Map(map) => Ok(Value::Boolean(!map.borrow().is_empty())),
        Value::Function { .. } | Value::NativeFunction { .. } => Ok(Value::Boolean(true)),
    }
}

pub fn native_push(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(arr) => {
            arr.borrow_mut().push(args[1].clone());
            Ok(Value::Null)
        }

        _ => Err(format!("push(): first argument must be array.")),
    }
}

pub fn native_pop(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(arr) => match arr.borrow_mut().pop() {
            Some(val) => Ok(val),
            None => Err(format!("pop(): cannot popping empty array.")),
        },
        _ => Err(format!("pop(): cannot popping non-array type.")),
    }
}

pub fn native_remove(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(arr) => match &args[1] {
            Value::Number(num) => {
                if *num >= 0.0 {
                    let i = *num as usize;
                    let len = arr.borrow().len();
                    if i < len {
                        Ok(arr.borrow_mut().remove(i))
                    } else {
                        Err(format!(
                            "remove(): index out of bounds. need index of {}, but only has {} indices.",
                            i, len
                        ))
                    }
                } else {
                    Err("remove(): index cannot be negative number".to_string())
                }
            }
            _ => Err("remove(): second argument must be a number.".to_string()),
        },
        Value::Map(map) => match &args[1] {
            Value::String(key) => match map.borrow_mut().shift_remove(key) {
                Some(val) => Ok(val),
                None => Ok(Value::Null),
            },
            _ => Err(
                "remove(): second argument must be a string key when removing from map."
                    .to_string(),
            ),
        },
        _ => Err("remove(): first argument must be array or map.".to_string()),
    }
}

pub fn native_len(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(arr) => Ok(Value::Number(arr.borrow().len() as f64)),
        Value::String(s) => Ok(Value::Number(s.chars().count() as f64)),
        Value::Map(map) => Ok(Value::Number(map.borrow().len() as f64)),
        _ => Err("len(): argument must be an array, string, or map.".to_string()),
    }
}

pub fn native_range(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() || args.len() > 3 {
        return Err(format!(
            "range(): expected 1, 2, or 3 arguments, got {}.",
            args.len()
        ));
    }

    let mut num_args = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        if let Value::Number(n) = arg {
            num_args.push(*n);
        } else {
            return Err(format!(
                "range(): argument {} must be a number, got '{}'.",
                i + 1,
                arg
            ));
        }
    }

    let (start, stop, step) = match num_args.len() {
        1 => (0.0, num_args[0], 1.0),
        2 => (num_args[0], num_args[1], 1.0),
        3 => (num_args[0], num_args[1], num_args[2]),
        _ => unreachable!(),
    };

    if step == 0.0 {
        return Err("range(): step argument must not be zero.".to_string());
    }

    let mut result = Vec::new();
    let mut current = start;

    if step > 0.0 {
        while current < stop {
            result.push(Value::Number(current));
            current += step;
        }
    } else {
        while current > stop {
            result.push(Value::Number(current));
            current += step;
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(result))))
}

pub fn native_keys(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Map(map) => {
            let keys: Vec<Value> = map
                .borrow()
                .keys()
                .map(|k| Value::String(k.clone()))
                .collect();
            Ok(Value::Array(Rc::new(RefCell::new(keys))))
        }
        _ => Err("keys(): argument must be a map.".to_string()),
    }
}

pub fn native_values(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Map(map) => {
            let values: Vec<Value> = map.borrow().values().cloned().collect();
            Ok(Value::Array(Rc::new(RefCell::new(values))))
        }
        _ => Err("values(): argument must be a map.".to_string()),
    }
}

pub fn native_has_key(args: Vec<Value>) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Map(map), Value::String(key)) => Ok(Value::Boolean(map.borrow().contains_key(key))),
        (Value::Map(_), _) => Err("hasKey(): second argument must be a string key.".to_string()),
        _ => Err("hasKey(): first argument must be a map.".to_string()),
    }
}

pub fn native_clear(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Map(map) => {
            map.borrow_mut().clear();
            Ok(Value::Null)
        }
        Value::Array(arr) => {
            arr.borrow_mut().clear();
            Ok(Value::Null)
        }
        _ => Err("clear(): argument must be a map or an array.".to_string()),
    }
}
