use crate::environment::Environment;
use crate::statements::is_error;
use crate::types::Object;
use downcast_rs::{impl_downcast, Downcast};

pub struct Program {
    pub statements: Vec<Box<dyn ProgramNode>>,
    pub environment: Environment,
    current_idx: usize,
}

impl Program {
    pub fn new(statements: Vec<Box<dyn ProgramNode>>) -> Program {
        return Program {
            statements,
            environment: Environment::new(),
            current_idx: 0,
        };
    }

    pub fn extend(&mut self, statements: Vec<Box<dyn ProgramNode>>) {
        self.statements.extend(statements);
    }

    fn total_statements(&self) -> usize {
        return self.statements.len();
    }

    fn update_env(&mut self, key: String, value: Box<dyn Object>) {
        self.environment.update(key, value);
    }

    pub fn eval(&mut self) -> Option<Box<dyn Object>> {
        if self.current_idx > self.total_statements() {
            return None;
        }

        let mut result: Option<Box<dyn Object>> = None;
        for idx in self.current_idx..self.total_statements() {
            // Get Result
            result = self.statements[idx].eval(&mut self.environment);

            if self.statements[idx].token_literal().unwrap() == "return" {
                return result;
            }

            if is_error(result.as_ref()) {
                return result;
            }

            // Update environment if Needed
            let env_update = self.statements[idx].update_env(&mut self.environment);
            if env_update.is_some() {
                let unwrapped = env_update.unwrap();
                for update in unwrapped {
                    self.update_env(update.0, update.1);
                }
            }

            // Move Along
            self.current_idx += 1;
        }

        return result;
    }
}

pub trait ProgramNode: Downcast {
    fn to_string(&self) -> String;
    fn token_literal(&self) -> Option<String>;
    fn eval(&self, env: &mut Environment) -> Option<Box<dyn Object>>;
    fn update_env(&self, env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>>;
    fn get_copy(&self) -> Box<dyn ProgramNode>;
}

impl_downcast!(ProgramNode);

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::Integer;

    struct Test {
        value: i64,
    }

    impl ProgramNode for Test {
        fn to_string(&self) -> String {
            return "".to_string();
        }
        fn token_literal(&self) -> Option<String> {
            return Some(format!("{}", self.value));
        }
        fn eval(&self, _env: &mut Environment) -> Option<Box<dyn Object>> {
            return Some(Box::new(Integer { value: self.value }));
        }

        fn update_env(&self, _env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
            return Some(vec![("Test".to_string(), Box::new(Integer { value: 5 }))]);
        }

        fn get_copy(&self) -> Box<dyn ProgramNode> {
            return Box::new(Test { value: self.value });
        }
    }

    #[test]
    fn test_program_node() {
        let statements: Vec<Box<dyn ProgramNode>> =
            vec![Box::new(Test { value: 5 }), Box::new(Test { value: 10 })];
        let mut program = Program::new(statements);

        program.eval();

        assert!(program
            .environment
            .list_keys()
            .contains(&&"Test".to_string()));
    }
}
