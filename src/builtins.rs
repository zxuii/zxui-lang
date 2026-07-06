use crate::object::Value;

use std::io::{self, Write};

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
