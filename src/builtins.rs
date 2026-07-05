use crate::object::Value;

use std::io::{self, Write};

pub fn native_println(args: Vec<Value>) -> Result<Value, String> {
    let _ = native_print(args);
    println!();
    Ok(Value::Null)
}

pub fn native_print(args: Vec<Value>) -> Result<Value, String> {
    for arg in &args {
        match arg {
            Value::Number(n) => {
                if n % 1.0 == 0.0 { print!("{}", *n as i64); }
                else { print!("{}", n); }
            }
            Value::Null => print!("null"),
            _ => print!("{:?}", arg),
        }
    }
    Ok(Value::Null)
}

pub fn native_readline(args: Vec<Value>) -> Result<Value, String> {
    // kita expect kalau args yang dibutuhkan sudah pasti didapatkan
    let mut input = String::new();

    print!("{}", args[0]);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to readline");

    Ok(Value::String(input))
}