use crate::types::Object;
use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ENVIRONMENT: Mutex<HashMap<&'static str, Box<dyn Object + Send>>> = {
        let hm: HashMap<&'static str, Box<dyn Object + Send>> = HashMap::new();
        Mutex::new(hm)
    };
}
