pub enum Type {
    INTEGER,
    BOOLEAN,
    NULL,
}

pub trait Object {
    fn type_(&self) -> Type;
    fn inspect(&self) -> String;
}

pub struct Integer {
    value: i64,
}

impl Object for Integer {
    fn type_(&self) -> Type {
        return Type::INTEGER;
    }
    fn inspect(&self) -> String {
        return format!("{}", self.value);
    }
}

pub struct Boolean {
    value: bool,
}

impl Object for Boolean {
    fn type_(&self) -> Type {
        return Type::BOOLEAN;
    }
    fn inspect(&self) -> String {
        return format!("{}", self.value);
    }
}

pub struct Null {}

impl Object for Null {
    fn type_(&self) -> Type {
        return Type::NULL;
    }
    fn inspect(&self) -> String {
        return "null".to_string();
    }
}
