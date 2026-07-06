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
    io::stdin().read_line(&mut input).expect("Failed to readline");

    let trimmed = input.trim_end_matches(['\n', '\r']).to_string();

    Ok(Value::String(trimmed))
}