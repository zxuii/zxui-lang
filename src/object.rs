#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Number(f64),
}

impl Object {
    fn equals(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::Null, Object::Null) => true,
            (_, Object::Null) => false,
            (Object::Null, _) => false,
            (Object::Number(left), Object::Number(right)) => left == right,
        }
    }
}
