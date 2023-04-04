extern crate downcast_rs;
use downcast_rs::{impl_downcast, Downcast};

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    INTEGER,
    BOOLEAN,
    NULL,
    ERROR,
}

pub trait Object: Downcast {
    fn type_(&self) -> Type;
    fn inspect(&self) -> String;
}

impl_downcast!(Object);

pub struct Integer {
    pub value: i64,
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
    pub value: bool,
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

pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn type_(&self) -> Type {
        return Type::ERROR;
    }
    fn inspect(&self) -> String {
        return self.message.to_owned();
    }
}
