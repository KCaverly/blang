use crate::environment::Environment;
use crate::types::{Integer, Object};
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
            result = self.statements[idx].eval(&self.environment);

            // Update environment if Needed
            let env_update = self.statements[idx].update_env(&self.environment);
            if env_update.is_some() {
                let unwrapped = env_update.unwrap();
                self.update_env(unwrapped.0, unwrapped.1);
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
    fn eval(&self, env: &Environment) -> Option<Box<dyn Object>>;
    fn update_env(&self, env: &Environment) -> Option<(String, Box<dyn Object>)>;
}

impl_downcast!(ProgramNode);

#[cfg(test)]
mod tests {

    struct Test {
        value: i64,
    }

    impl ProgramNode for Test {
        fn to_string(&self) -> String {
            return "".to_string();
        }
        fn token_literal(&self) -> Option<String> {
            return None;
        }
        fn eval(&self, env: &Environment) -> Option<Box<dyn Object>> {
            return Some(Box::new(Integer { value: self.value }));
        }

        fn update_env(&self, env: &Environment) -> Option<(String, Box<dyn Object>)> {
            return Some(("Test".to_string(), Box::new(Integer { value: 5 })));
        }
    }

    use super::*;

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
