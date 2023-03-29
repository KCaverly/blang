use crate::ast::{Node, Program, Statement};
use crate::types::{Boolean, Integer, Null, Object};

pub fn eval(node: Box<dyn Statement>) -> Box<dyn Object> {
    let struct_name = &*node.struct_name();

    match struct_name {
        "Program" => eval_statements(node),
        _ => panic!("NOT A PROGRAM!"),
    };

    return Box::new(Null {});
}

pub fn eval_statements(node: Box<dyn Statement>) {
    // let mut result: Box<dyn Object>;
    // let statements = node.downcast_ref::<Program>().unwrap().statements;
    // for statement in &statements {
    //     result = eval(Box::new(statement.as_ref().to_owned()));
    //
    // jjj}
    // return result;
}
