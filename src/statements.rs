use crate::environment::Environment;
use crate::program::ProgramNode;
use crate::token::Token;
use crate::types::{Boolean, Error, Function, Integer, Object, Type};

pub fn is_error(object: Option<&Box<dyn Object>>) -> bool {
    if object.is_some() {
        if object.as_ref().unwrap().type_() == Type::ERROR {
            return true;
        }
    }
    return false;
}

pub struct LetStatement {
    pub token: Token,
    pub name: Box<dyn ProgramNode>,
    pub value: Box<dyn ProgramNode>,
}

impl LetStatement {
    pub fn new(
        token: Token,
        name: Box<dyn ProgramNode>,
        value: Box<dyn ProgramNode>,
    ) -> LetStatement {
        return LetStatement { token, name, value };
    }
}

impl ProgramNode for LetStatement {
    fn to_string(&self) -> String {
        return format!(
            "{} {} = {}",
            self.token_literal().unwrap(),
            self.name.to_string(),
            self.value.to_string()
        );
    }

    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }

    fn eval(&self, _env: &mut Environment) -> Option<Box<dyn Object>> {
        return None;
    }

    fn update_env(&self, env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        let result = self.value.eval(env);
        if result.is_some() {
            return Some(vec![(self.name.to_string(), result.unwrap())]);
        }

        return None;
    }

    fn get_copy(&self) -> Box<dyn ProgramNode> {
        return Box::new(LetStatement {
            token: self.token.clone(),
            name: self.name.get_copy(),
            value: self.value.get_copy(),
        });
    }
}

pub struct ReturnStatement {
    token: Token,
    value: Box<dyn ProgramNode>,
}

impl ReturnStatement {
    pub fn new(token: Token, value: Box<dyn ProgramNode>) -> ReturnStatement {
        return ReturnStatement { token, value };
    }
}

impl ProgramNode for ReturnStatement {
    fn to_string(&self) -> String {
        return format!(
            "{} {};",
            self.token_literal().unwrap(),
            self.value.to_string()
        );
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn eval(&self, env: &mut Environment) -> Option<Box<dyn Object>> {
        return self.value.eval(env);
    }
    fn update_env(&self, _env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        return None;
    }

    fn get_copy(&self) -> Box<dyn ProgramNode> {
        return Box::new(ReturnStatement {
            token: self.token.clone(),
            value: self.value.get_copy(),
        });
    }
}

pub struct ExpressionStatement {
    token: Token,
    pub expression: Box<dyn ProgramNode>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn ProgramNode>) -> ExpressionStatement {
        return ExpressionStatement { token, expression };
    }
}

impl ProgramNode for ExpressionStatement {
    fn to_string(&self) -> String {
        return self.expression.to_string();
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn eval(&self, env: &mut Environment) -> Option<Box<dyn Object>> {
        return self.expression.eval(env);
    }
    fn update_env(&self, env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        return self.expression.update_env(env);
    }

    fn get_copy(&self) -> Box<dyn ProgramNode> {
        return Box::new(ExpressionStatement {
            token: self.token.clone(),
            expression: self.expression.get_copy(),
        });
    }
}

pub struct BlockStatement {
    token: Token,
    pub statements: Vec<Box<dyn ProgramNode>>,
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Box<dyn ProgramNode>>) -> BlockStatement {
        return BlockStatement { token, statements };
    }
}

impl ProgramNode for BlockStatement {
    fn to_string(&self) -> String {
        let mut str: Vec<String> = Vec::new();
        for statement in &self.statements {
            str.push(format!("{};", statement.to_string()));
        }
        return str.join(" ");
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn eval(&self, env: &mut Environment) -> Option<Box<dyn Object>> {
        let mut result: Option<Box<dyn Object>> = None;
        for statement in &self.statements {
            result = statement.eval(env);

            if statement.token_literal().unwrap() == "return" {
                return result;
            }

            if is_error(result.as_ref()) {
                return result;
            }

            let env_update = statement.update_env(env);
            if env_update.is_some() {
                let unwrapped = env_update.unwrap();
                for update in unwrapped {
                    env.update(update.0, update.1);
                }
            }
        }

        return result;
    }
    fn update_env(&self, env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        let mut updates: Vec<(String, Box<dyn Object>)> = vec![];
        let mut result: Option<Box<dyn Object>>;
        for statement in &self.statements {
            result = statement.eval(env);

            if statement.token_literal().unwrap() == "return" {
                return Some(updates);
            }

            if is_error(result.as_ref()) {
                return Some(updates);
            }

            let env_update = statement.update_env(env);
            if env_update.is_some() {
                let unwrapped = env_update.unwrap();
                for update in unwrapped {
                    env.update(update.0.clone(), update.1.get_box());
                    updates.push((update.0, update.1));
                }
            }
        }

        return Some(updates);
    }

    fn get_copy(&self) -> Box<dyn ProgramNode> {
        let mut statements: Vec<Box<dyn ProgramNode>> = vec![];
        for statement in &self.statements {
            statements.push(statement.get_copy());
        }
        return Box::new(BlockStatement {
            token: self.token.clone(),
            statements: statements,
        });
    }
}

pub struct IdentifierExpression {
    pub token: Token,
    pub value: String,
}

impl IdentifierExpression {
    pub fn new(token: Token, value: String) -> IdentifierExpression {
        return IdentifierExpression { token, value };
    }
}

impl ProgramNode for IdentifierExpression {
    fn to_string(&self) -> String {
        return self.value.clone();
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn eval(&self, env: &mut Environment) -> Option<Box<dyn Object>> {
        return Some(env.get(&self.value));
    }
    fn update_env(&self, _env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        return None;
    }

    fn get_copy(&self) -> Box<dyn ProgramNode> {
        return Box::new(IdentifierExpression {
            token: self.token.clone(),
            value: self.value.clone(),
        });
    }
}

pub struct IntegerLiteralExpression {
    token: Token,
    pub value: i64,
}

impl IntegerLiteralExpression {
    pub fn new(token: Token, value: i64) -> IntegerLiteralExpression {
        return IntegerLiteralExpression { token, value };
    }
}

impl ProgramNode for IntegerLiteralExpression {
    fn to_string(&self) -> String {
        return self.value.clone().to_string();
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn eval(&self, _env: &mut Environment) -> Option<Box<dyn Object>> {
        return Some(Box::new(Integer { value: self.value }));
    }
    fn update_env(&self, _env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        return None;
    }
    fn get_copy(&self) -> Box<dyn ProgramNode> {
        return Box::new(IntegerLiteralExpression {
            token: self.token.clone(),
            value: self.value.clone(),
        });
    }
}

pub struct BooleanExpression {
    token: Token,
    value: bool,
}

impl BooleanExpression {
    pub fn new(token: Token, value: bool) -> BooleanExpression {
        return BooleanExpression { token, value };
    }
}

impl ProgramNode for BooleanExpression {
    fn to_string(&self) -> String {
        return self.value.to_string();
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn eval(&self, _env: &mut Environment) -> Option<Box<dyn Object>> {
        return Some(Box::new(Boolean { value: self.value }));
    }
    fn update_env(&self, _env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        return None;
    }

    fn get_copy(&self) -> Box<dyn ProgramNode> {
        return Box::new(BooleanExpression {
            token: self.token.clone(),
            value: self.value.clone(),
        });
    }
}

pub struct PrefixExpression {
    token: Token,
    operator: String,
    right: Box<dyn ProgramNode>,
}

impl PrefixExpression {
    pub fn new(token: Token, operator: String, right: Box<dyn ProgramNode>) -> PrefixExpression {
        return PrefixExpression {
            token,
            operator,
            right,
        };
    }
}

impl ProgramNode for PrefixExpression {
    fn to_string(&self) -> String {
        return format!("({}{})", self.operator, self.right.to_string());
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn eval(&self, env: &mut Environment) -> Option<Box<dyn Object>> {
        let right_eval = self.right.eval(env);
        let right_result = right_eval.as_ref().unwrap();
        if is_error(right_eval.as_ref()) {
            return right_eval;
        }
        let right_type = right_result.type_();

        let op = self.operator.as_str();
        match op {
            "!" => {
                let res: bool;
                if right_type == Type::BOOLEAN {
                    if right_result.downcast_ref::<Boolean>().unwrap().value {
                        res = false;
                    } else {
                        res = true;
                    }
                } else if right_type == Type::NULL {
                    res = false;
                } else {
                    res = false;
                }
                return Some(Box::new(Boolean { value: res }));
            }
            "-" => {
                if right_type == Type::INTEGER {
                    let val = right_result.downcast_ref::<Integer>().unwrap().value;
                    return Some(Box::new(Integer { value: -val }));
                } else {
                    return Some(Box::new(Error {
                        message: format!("invalid type: -{:?}", right_type),
                    }));
                }
            }
            _ => {
                return Some(Box::new(Error {
                    message: format!("unknown operator: {:?}", op),
                }));
            }
        }
    }

    fn update_env(&self, _env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        return None;
    }

    fn get_copy(&self) -> Box<dyn ProgramNode> {
        return Box::new(PrefixExpression {
            token: self.token.clone(),
            operator: self.operator.clone(),
            right: self.right.get_copy(),
        });
    }
}

pub struct InfixExpression {
    token: Token,
    pub left: Box<dyn ProgramNode>,
    pub operator: String,
    pub right: Box<dyn ProgramNode>,
}

impl InfixExpression {
    pub fn new(
        token: Token,
        left: Box<dyn ProgramNode>,
        operator: String,
        right: Box<dyn ProgramNode>,
    ) -> InfixExpression {
        return InfixExpression {
            token,
            left,
            operator,
            right,
        };
    }
}

impl ProgramNode for InfixExpression {
    fn to_string(&self) -> String {
        return format!(
            "({} {} {})",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        );
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn eval(&self, env: &mut Environment) -> Option<Box<dyn Object>> {
        // Check Left
        let left_eval = self.left.eval(env);
        if is_error(left_eval.as_ref()) {
            return left_eval;
        }
        let left_result = left_eval.unwrap();

        // Check right
        let right_eval = self.right.eval(env);
        if is_error(right_eval.as_ref()) {
            return right_eval;
        }
        let right_result = right_eval.unwrap();

        if left_result.type_() == Type::INTEGER && right_result.type_() == Type::INTEGER {
            let left_int = left_result.downcast_ref::<Integer>().unwrap();
            let right_int = right_result.downcast_ref::<Integer>().unwrap();

            let res: Option<Box<dyn Object>> = match self.operator.as_str() {
                "-" => Some(Box::new(Integer {
                    value: left_int.value - right_int.value,
                })),
                "+" => Some(Box::new(Integer {
                    value: left_int.value + right_int.value,
                })),
                "/" => Some(Box::new(Integer {
                    value: left_int.value / right_int.value,
                })),
                "*" => Some(Box::new(Integer {
                    value: left_int.value * right_int.value,
                })),
                ">" => Some(Box::new(Boolean {
                    value: left_int.value > right_int.value,
                })),
                "<" => Some(Box::new(Boolean {
                    value: left_int.value < right_int.value,
                })),
                "==" => Some(Box::new(Boolean {
                    value: left_int.value == right_int.value,
                })),
                "!=" => Some(Box::new(Boolean {
                    value: left_int.value != right_int.value,
                })),
                _ => None,
            };
            return res;
        } else if left_result.type_() == Type::BOOLEAN && right_result.type_() == Type::BOOLEAN {
            let left_bool = left_result.downcast_ref::<Boolean>().unwrap();
            let right_bool = right_result.downcast_ref::<Boolean>().unwrap();

            let res: Option<Box<dyn Object>> = match self.operator.as_str() {
                "==" => Some(Box::new(Boolean {
                    value: left_bool.value == right_bool.value,
                })),
                "!=" => Some(Box::new(Boolean {
                    value: left_bool.value != right_bool.value,
                })),
                _ => None,
            };
            return res;
        } else {
            return Some(Box::new(Error {
                message: format!(
                    "type mismatch: {:?} {} {:?}",
                    left_result.type_(),
                    self.operator.as_str(),
                    right_result.type_()
                ),
            }));
        }
    }
    fn update_env(&self, _env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        return None;
    }

    fn get_copy(&self) -> Box<dyn ProgramNode> {
        return Box::new(InfixExpression {
            token: self.token.clone(),
            left: self.left.get_copy(),
            operator: self.operator.clone(),
            right: self.right.get_copy(),
        });
    }
}

pub struct IfExpression {
    token: Token,
    pub condition: Box<dyn ProgramNode>,
    pub consequence: Box<dyn ProgramNode>,
    pub alternative: Option<Box<dyn ProgramNode>>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Box<dyn ProgramNode>,
        consequence: Box<dyn ProgramNode>,
        alternative: Option<Box<dyn ProgramNode>>,
    ) -> IfExpression {
        return IfExpression {
            token,
            condition,
            consequence,
            alternative,
        };
    }
}

impl ProgramNode for IfExpression {
    fn to_string(&self) -> String {
        if self.alternative.is_some() {
            let alt = self.alternative.as_ref().unwrap();
            return format!(
                "if {} {} else {}",
                self.condition.to_string(),
                self.consequence.to_string(),
                alt.to_string()
            );
        } else {
            return format!(
                "if {} {}",
                self.condition.to_string(),
                self.consequence.to_string()
            );
        }
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn eval(&self, env: &mut Environment) -> Option<Box<dyn Object>> {
        let condition_result = self.condition.eval(env);
        if is_error(condition_result.as_ref()) {
            return condition_result;
        }

        let use_first: bool;
        if condition_result.is_some() {
            let unwrapped = condition_result.unwrap();
            if &unwrapped.type_() == &Type::BOOLEAN {
                use_first = unwrapped.downcast_ref::<Boolean>().unwrap().value;
            } else {
                use_first = true;
            }
        } else {
            use_first = false;
        }

        if use_first {
            let res = self.consequence.eval(env);
            return res;
        } else if self.alternative.is_some() {
            let unwrapped = self.alternative.as_ref().unwrap();
            return unwrapped.eval(env);
        } else {
            return None;
        }
    }
    fn update_env(&self, _env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        return None;
    }

    fn get_copy(&self) -> Box<dyn ProgramNode> {
        let alt: Option<Box<dyn ProgramNode>>;
        if self.alternative.is_some() {
            alt = Some(self.alternative.as_ref().unwrap().get_copy());
        } else {
            alt = None;
        }
        return Box::new(IfExpression {
            token: self.token.clone(),
            condition: self.condition.get_copy(),
            consequence: self.consequence.get_copy(),
            alternative: alt,
        });
    }
}

pub struct FunctionLiteralExpression {
    token: Token,
    pub parameters: Vec<Box<dyn ProgramNode>>,
    pub body: Box<dyn ProgramNode>,
}

impl FunctionLiteralExpression {
    pub fn new(
        token: Token,
        parameters: Vec<Box<dyn ProgramNode>>,
        body: Box<dyn ProgramNode>,
    ) -> FunctionLiteralExpression {
        return FunctionLiteralExpression {
            token,
            parameters,
            body,
        };
    }
}

impl ProgramNode for FunctionLiteralExpression {
    fn to_string(&self) -> String {
        return format!(
            "fn({}) {{ {} }}",
            self.parameters
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.body.to_string()
        );
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.clone();
    }
    fn eval(&self, _env: &mut Environment) -> Option<Box<dyn Object>> {
        let mut params: Vec<Box<dyn ProgramNode>> = vec![];
        for param in &self.parameters {
            params.push(param.get_copy());
        }
        return Some(Box::new(Function {
            body: self.body.get_copy(),
            env: _env.get_copy(),
            parameters: params,
        }));
    }

    fn update_env(&self, _env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        todo!();
    }
    fn get_copy(&self) -> Box<dyn ProgramNode> {
        let mut params: Vec<Box<dyn ProgramNode>> = vec![];
        for param in &self.parameters {
            params.push(param.get_copy());
        }
        return Box::new(FunctionLiteralExpression {
            token: self.token.clone(),
            parameters: params,
            body: self.body.get_copy(),
        });
    }
}

pub struct CallExpression {
    token: Token,
    pub function: Box<dyn ProgramNode>,
    pub arguments: Vec<Box<dyn ProgramNode>>,
}

impl CallExpression {
    pub fn new(
        token: Token,
        function: Box<dyn ProgramNode>,
        arguments: Vec<Box<dyn ProgramNode>>,
    ) -> CallExpression {
        return CallExpression {
            token,
            function,
            arguments,
        };
    }
}

impl ProgramNode for CallExpression {
    fn to_string(&self) -> String {
        return format!(
            "{}({})",
            self.function.to_string(),
            self.arguments
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.clone();
    }
    fn eval(&self, _env: &mut Environment) -> Option<Box<dyn Object>> {
        let mut scoped_env = _env.get_copy();

        // Get Function Object
        let og_fn = self.function.eval(&mut scoped_env).unwrap();
        let og_fn_un = og_fn.downcast_ref::<Function>().unwrap();

        // Evaluate Arguments
        for idx in 0..self.arguments.len() {
            let eval_ = self.arguments[idx].eval(&mut scoped_env).unwrap();
            scoped_env.update(og_fn_un.parameters[idx].token_literal().unwrap(), eval_);
        }

        let unwrapped = self.function.eval(&mut scoped_env).unwrap();
        let fn_ = unwrapped.downcast_ref::<Function>().unwrap();

        let result = fn_.body.eval(&mut scoped_env);

        return result;
    }
    fn update_env(&self, _env: &mut Environment) -> Option<Vec<(String, Box<dyn Object>)>> {
        return None;
    }
    fn get_copy(&self) -> Box<dyn ProgramNode> {
        let mut args: Vec<Box<dyn ProgramNode>> = vec![];
        for arg in &self.arguments {
            args.push(arg.get_copy());
        }
        return Box::new(CallExpression {
            token: self.token.clone(),
            function: self.function.get_copy(),
            arguments: args,
        });
    }
}
