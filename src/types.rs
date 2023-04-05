extern crate downcast_rs;
use downcast_rs::{impl_downcast, Downcast};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    INTEGER,
    BOOLEAN,
    NULL,
    ERROR,
    FUNCTION,
}

pub trait Object: Downcast + Send {
    fn type_(&self) -> Type;
    fn inspect(&self) -> String;
    fn get_box(&self) -> Box<dyn Object>;
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
    fn get_box(&self) -> Box<dyn Object> {
        return Box::new(Integer { value: self.value });
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

    fn get_box(&self) -> Box<dyn Object> {
        return Box::new(Boolean { value: self.value });
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
    fn get_box(&self) -> Box<dyn Object> {
        return Box::new(Null {});
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

    fn get_box(&self) -> Box<dyn Object> {
        return Box::new(Error {
            message: self.message.clone(),
        });
    }
}
