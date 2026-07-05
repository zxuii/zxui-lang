use crate::object::Value;

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