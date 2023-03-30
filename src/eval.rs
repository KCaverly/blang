use crate::ast::{Node, Program};
use crate::types::{Boolean, Integer, Null, Object};

pub fn eval(node: &Box<dyn Node>) -> Box<dyn Object> {
    // let struct_name = &*node.struct_name();
    //
    // match struct_name {
    //     "Program" => eval_statements(node),
    //     _ => panic!("NOT A PROGRAM!"),
    // };

    return Box::new(Null {});
}

pub fn eval_statements(node: &Box<dyn Node>) -> Option<Box<dyn Object>> {
    let mut result: Option<Box<dyn Object>> = None;
    let statements = &node.downcast_ref::<Program>().unwrap().statements;
    for statement in statements {
        result = Some(eval(statement));
    }
    return result;
}
