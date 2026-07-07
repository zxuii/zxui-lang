use crate::object::Value;

use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

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
    // kita expect kalau args yang dibutuhkan sudah pasti didapatkan
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

        _ => Err(format!(
            "number(): type '{}' cannot be converted to number.",
            args[0]
        )),
    }
}

pub fn native_string(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::String(format!("{}", args[0])))
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
        _ => Err("remove(): first argument must be array.".to_string()),
    }
}

pub fn native_len(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(arr) => Ok(Value::Number(arr.borrow().len() as f64)),
        Value::String(s) => Ok(Value::Number(s.chars().count() as f64)),
        _ => Err("len(): argument must be an array or string.".to_string()),
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
