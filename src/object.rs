#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Number(f64),
}

impl Value {
    fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (_, Value::Null) => false,
            (Value::Null, _) => false,
            (Value::Number(left), Value::Number(right)) => left == right,
        }
    }
}
