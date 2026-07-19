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
    let mut input = String::new();

    print!("{}", args[0]);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");

    let trimmed = input.trim_end_matches(['\n', '\r']).to_string();

    Ok(Value::String(trimmed))
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