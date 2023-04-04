extern crate downcast_rs;
extern crate lazy_static;

use crate::environment::ENVIRONMENT;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::types::{Boolean, Error, Integer, Object, Type};
use downcast_rs::{impl_downcast, Downcast};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub trait Node: Downcast {
    fn token_literal(&self) -> Option<String>;
    fn to_string(&self) -> String;
    fn eval(&self) -> Option<Box<dyn Object>>;
}

impl_downcast!(Node);

////////////
// Helper //
////////////

fn is_error(object: Option<&Box<dyn Object>>) -> bool {
    if object.is_some() {
        if object.as_ref().unwrap().type_() == Type::ERROR {
            return true;
        }
    }
    return false;
}

/////////////
// Program //
/////////////

pub struct Program {
    pub statements: Vec<Box<dyn Node>>,
}

impl Node for Program {
    fn token_literal(&self) -> Option<String> {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return None;
        }
    }

    fn to_string(&self) -> String {
        let mut str: Vec<String> = Vec::new();
        for statement in &self.statements {
            str.push(statement.to_string());
        }
        return str.join(" ");
    }

    fn eval(&self) -> Option<Box<dyn Object>> {
        let mut result: Option<Box<dyn Object>> = None;
        for statement in &self.statements {
            result = statement.eval();

            if statement.as_ref().token_literal().unwrap() == "return" {
                break;
            }

            if result.as_ref().unwrap().type_() == Type::ERROR {
                break;
            }
        }
        return result;
    }
}

///////////////
// Statement //
///////////////

struct LetStatement {
    token: Token,
    name: IdentifierExpression,
    value: Box<dyn Node>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }

    fn to_string(&self) -> String {
        return format!(
            "{} {} = {}",
            self.token_literal().unwrap(),
            self.name.to_string(),
            self.value.to_string()
        )
        .to_string();
    }

    fn eval(&self) -> Option<Box<dyn Object>> {
        if ENVIRONMENT
            .lock()
            .unwrap()
            .contains_key(self.name.token_literal().unwrap().as_str())
        {
            println!("HAS THE KEY ALREADY!");
        } else {
            // ENVIRONMENT.lock().unwrap()[self.name.token_literal().unwrap().as_str()] =
            //     self.value.eval().unwrap();
            println!("MISSING KEY!");
        }
        return None;
    }
}

struct ReturnStatement {
    token: Token,
    value: Box<dyn Node>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }

    fn to_string(&self) -> String {
        return format!(
            "{} {};",
            self.token_literal().unwrap(),
            self.value.to_string()
        );
    }

    fn eval(&self) -> Option<Box<dyn Object>> {
        return self.value.eval();
    }
}

struct ExpressionStatement {
    token: Token,
    expression: Box<dyn Node>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        return self.expression.to_string();
    }
    fn eval(&self) -> Option<Box<dyn Object>> {
        return self.expression.eval();
    }
}

struct BlockStatement {
    token: Token,
    statements: Vec<Box<dyn Node>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        let mut str: Vec<String> = Vec::new();
        for statement in &self.statements {
            str.push(format!("{};", statement.to_string()));
        }
        return str.join(" ");
    }
    fn eval(&self) -> Option<Box<dyn Object>> {
        let mut result: Option<Box<dyn Object>> = None;
        for statement in &self.statements {
            result = statement.eval();
            if statement.as_ref().token_literal().unwrap() == "return" {
                break;
            }

            if result.as_ref().unwrap().type_() == Type::ERROR {
                break;
            }
        }
        return result;
    }
}

////////////////
// Expression //
////////////////

struct IdentifierExpression {
    token: Token,
    value: String,
}

impl Node for IdentifierExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        return self.value.clone();
    }
    fn eval(&self) -> Option<Box<dyn Object>> {
        return None;
    }
}

struct IntegerLiteralExpression {
    token: Token,
    value: i64,
}

impl Node for IntegerLiteralExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        return self.value.clone().to_string();
    }
    fn eval(&self) -> Option<Box<dyn Object>> {
        return Some(Box::new(Integer { value: self.value }));
    }
}

struct BooleanExpression {
    token: Token,
    value: bool,
}

impl Node for BooleanExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        return self.value.to_string();
    }
    fn eval(&self) -> Option<Box<dyn Object>> {
        return Some(Box::new(Boolean { value: self.value }));
    }
}

struct PrefixExpression {
    token: Token,
    operator: String,
    right: Box<dyn Node>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        return format!("({}{})", self.operator, self.right.to_string());
    }
    fn eval(&self) -> Option<Box<dyn Object>> {
        let right_eval = self.right.eval();
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
                        message: format!("unknown operator: -{:?}", right_type),
                    }));
                }
            }
            _ => {
                // I think this error may be impossible given upstream calls...
                return Some(Box::new(Error {
                    message: format!("unknown operator: {:?}", op),
                }));
            }
        };
    }
}

struct InfixExpression {
    token: Token,
    left: Box<dyn Node>,
    operator: String,
    right: Box<dyn Node>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        return format!(
            "({} {} {})",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        );
    }
    fn eval(&self) -> Option<Box<dyn Object>> {
        // Check left
        let left_eval = self.left.eval();

        if is_error(left_eval.as_ref()) {
            return left_eval;
        }
        let left_result = left_eval.unwrap();

        // Check right
        let right_eval = self.right.eval();
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
}

struct IfExpression {
    token: Token,
    condition: Box<dyn Node>,
    consequence: Box<dyn Node>,
    alternative: Option<Box<dyn Node>>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
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
    fn eval(&self) -> Option<Box<dyn Object>> {
        let condition_result = self.condition.eval();
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
            let res = self.consequence.eval();
            return res;
        } else if self.alternative.is_some() {
            let unwrapped = self.alternative.as_ref().unwrap();
            return unwrapped.eval();
        } else {
            return None;
        }
    }
}

struct FunctionLiteralExpression {
    token: Token,
    parameters: Vec<IdentifierExpression>,
    body: Box<dyn Node>,
}

impl Node for FunctionLiteralExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.clone();
    }
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
    fn eval(&self) -> Option<Box<dyn Object>> {
        return None;
    }
}

struct CallExpression {
    token: Token,
    function: Box<dyn Node>,
    arguments: Vec<Box<dyn Node>>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.clone();
    }
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

    fn eval(&self) -> Option<Box<dyn Object>> {
        return None;
    }
}

//////////////////
// Precendences //
//////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PrecedenceType {
    LOWEST = 0,
    EQUALS = 1,
    LESSGREATER = 2,
    SUM = 3,
    PRODUCT = 4,
    PREFIX = 5,
    CALL = 6,
}

lazy_static! {
    static ref PRECEDENCE_MAP: HashMap<TokenType, PrecedenceType> = HashMap::from([
        (TokenType::EQ, PrecedenceType::EQUALS),
        (TokenType::NEQ, PrecedenceType::EQUALS),
        (TokenType::LT, PrecedenceType::LESSGREATER),
        (TokenType::GT, PrecedenceType::LESSGREATER),
        (TokenType::PLUS, PrecedenceType::SUM),
        (TokenType::MINUS, PrecedenceType::SUM),
        (TokenType::SLASH, PrecedenceType::PRODUCT),
        (TokenType::ASTERISK, PrecedenceType::PRODUCT),
        (TokenType::LPAREN, PrecedenceType::CALL)
    ]);
}

////////////
// Parser //
////////////

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        let parser = Parser {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
        };

        return parser;
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn current_token_is(&self, token_type: &TokenType) -> bool {
        if &self.current_token.token_type == token_type {
            return true;
        } else {
            return false;
        }
    }

    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        if &self.peek_token.token_type == token_type {
            return true;
        } else {
            return false;
        }
    }

    fn expect_peek(&mut self, token_type: &TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        } else {
            let msg = format!(
                "Expected next token to be {:?}, got {:?} instead",
                &self.peek_token.token_type, token_type
            );
            self.errors.push(msg);
            return false;
        }
    }

    fn current_precedence(&mut self) -> &PrecedenceType {
        return &PRECEDENCE_MAP[&self.current_token.clone().token_type.clone()];
    }

    fn peek_precedence(&mut self) -> PrecedenceType {
        if PRECEDENCE_MAP.contains_key(&self.peek_token.clone().token_type) {
            return PRECEDENCE_MAP[&self.peek_token.clone().token_type.clone()];
        }
        return PrecedenceType::LOWEST;
    }

    pub fn parse(&mut self) -> Program {
        // Create Blank Program to Start
        let mut program = Program { statements: vec![] };

        // Iterate through all token in the Lexer
        // TODO: We have to handle semicolons at some point
        while !self.current_token_is(&TokenType::EOF) {
            if !self.current_token_is(&TokenType::SEMICOLON) {
                let statement = self.parse_statement();
                program.statements.push(statement);
            }

            self.next_token();
        }

        return program;
    }

    fn parse_statement(&mut self) -> Box<dyn Node> {
        let token_type = self.current_token.token_type;
        let statement = match token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            TokenType::INT => self.parse_expression_statement(),
            TokenType::BANG => self.parse_expression_statement(),
            TokenType::MINUS => self.parse_expression_statement(),
            TokenType::IDENT => self.parse_expression_statement(),
            TokenType::TRUE => self.parse_expression_statement(),
            TokenType::FALSE => self.parse_expression_statement(),
            TokenType::LPAREN => self.parse_expression_statement(),
            TokenType::IF => self.parse_expression_statement(),
            TokenType::FUNCTION => self.parse_expression_statement(),
            _ => panic!("PANIC!"),
        };

        return statement;
    }

    fn parse_let_statement(&mut self) -> Box<dyn Node> {
        let og_token = self.current_token.clone();

        if !self.expect_peek(&TokenType::IDENT) {
            panic!("Identifier structured incorrectly");
        }

        let name = IdentifierExpression {
            token: self.current_token.clone(),
            value: self.current_token.clone().literal.unwrap(),
        };

        if !self.expect_peek(&TokenType::ASSIGN) {
            panic!("Identifier structured incorrectly");
        } else {
            self.next_token();
        }

        return Box::new(LetStatement {
            token: og_token,
            name,
            value: self.parse_expression(PrecedenceType::LOWEST),
        });
    }
    fn parse_return_statement(&mut self) -> Box<dyn Node> {
        let og_token = self.current_token.clone();
        self.next_token();

        return Box::new(ReturnStatement {
            token: og_token,
            value: self.parse_expression(PrecedenceType::LOWEST),
        });
    }

    fn parse_expression_statement(&mut self) -> Box<dyn Node> {
        let expr = self.parse_expression(PrecedenceType::LOWEST);
        return Box::new(ExpressionStatement {
            token: self.current_token.clone(),
            expression: expr,
        });
    }

    fn parse_block_statement(&mut self) -> Box<dyn Node> {
        let og_token = self.current_token.clone();
        let mut statements = vec![];

        self.next_token();

        while !self.current_token_is(&TokenType::RBRACE) && !self.current_token_is(&TokenType::EOF)
        {
            if !self.current_token_is(&TokenType::SEMICOLON) {
                let statement = self.parse_statement();
                statements.push(statement);
            }
            self.next_token();
        }

        return Box::new(BlockStatement {
            token: og_token,
            statements,
        });
    }

    fn parse_expression(&mut self, precedence: PrecedenceType) -> Box<dyn Node> {
        let token_type = self.current_token.token_type;

        // Parse Left Side of Expression
        let left_expr = match token_type {
            TokenType::INT => Some(self.parse_integer_expression()),
            TokenType::BANG => Some(self.parse_prefix_expression()),
            TokenType::MINUS => Some(self.parse_prefix_expression()),
            TokenType::FUNCTION => Some(self.parse_function_expression()),
            TokenType::IDENT => Some(self.parse_identifier_expression()),
            TokenType::TRUE => Some(self.parse_boolean_expression()),
            TokenType::FALSE => Some(self.parse_boolean_expression()),
            TokenType::LPAREN => Some(self.parse_grouped_expression()),
            TokenType::IF => Some(self.parse_if_expression()),

            _ => None,
        };

        if left_expr.is_none() {
            panic!("LEFT EXPR IS NONE!");
        } else {
            let mut expr = left_expr.unwrap();
            while !self.peek_token_is(&TokenType::SEMICOLON) && precedence < self.peek_precedence()
            {
                self.next_token();
                let next_token = self.current_token.clone().token_type;
                expr = match next_token {
                    TokenType::IDENT => self.parse_identifier_expression(),
                    TokenType::PLUS => self.parse_infix_expression(expr),
                    TokenType::MINUS => self.parse_infix_expression(expr),
                    TokenType::SLASH => self.parse_infix_expression(expr),
                    TokenType::ASTERISK => self.parse_infix_expression(expr),
                    TokenType::EQ => self.parse_infix_expression(expr),
                    TokenType::NEQ => self.parse_infix_expression(expr),
                    TokenType::GT => self.parse_infix_expression(expr),
                    TokenType::LT => self.parse_infix_expression(expr),
                    TokenType::LPAREN => self.parse_call_expression(expr),
                    _ => panic!("PANICKING!"),
                };
            }
            return expr;
        }
    }

    fn parse_integer_expression(&mut self) -> Box<dyn Node> {
        let expr = Box::new(IntegerLiteralExpression {
            token: self.current_token.clone(),
            value: self
                .current_token
                .clone()
                .literal
                .unwrap()
                .parse::<i64>()
                .unwrap(),
        });
        return expr;
    }
    fn parse_identifier_expression(&mut self) -> Box<dyn Node> {
        let expr = Box::new(IdentifierExpression {
            token: self.current_token.clone(),
            value: self.current_token.clone().literal.unwrap(),
        });
        return expr;
    }

    fn parse_boolean_expression(&mut self) -> Box<dyn Node> {
        let expr = Box::new(BooleanExpression {
            token: self.current_token.clone(),
            value: self
                .current_token
                .clone()
                .literal
                .unwrap()
                .parse::<bool>()
                .unwrap(),
        });
        return expr;
    }

    fn parse_prefix_expression(&mut self) -> Box<dyn Node> {
        let og_token = self.current_token.clone();
        self.next_token();

        return Box::new(PrefixExpression {
            token: og_token.clone(),
            operator: og_token.literal.clone().unwrap(),
            right: self.parse_expression(PrecedenceType::PREFIX),
        });
    }
    fn parse_infix_expression(&mut self, left: Box<dyn Node>) -> Box<dyn Node> {
        let og_token = self.current_token.clone();

        let precedence = PRECEDENCE_MAP[&og_token.token_type];
        self.next_token();
        return Box::new(InfixExpression {
            token: og_token.clone(),
            left,
            operator: og_token.clone().literal.unwrap(),
            right: self.parse_expression(precedence),
        });
    }

    fn parse_grouped_expression(&mut self) -> Box<dyn Node> {
        self.next_token();

        let expr = self.parse_expression(PrecedenceType::LOWEST);
        if !self.expect_peek(&TokenType::RPAREN) {
            panic!("{}", "DOES NOT INCLUDE RPAREN");
        }

        return expr;
    }

    fn parse_if_expression(&mut self) -> Box<dyn Node> {
        let og_token = self.current_token.clone();
        if !self.expect_peek(&TokenType::LPAREN) {
            panic!("INVALID!");
        }

        self.next_token();

        let condition = self.parse_expression(PrecedenceType::LOWEST);

        if !self.expect_peek(&TokenType::RPAREN) {
            panic!("INVALID 2");
        }

        if !self.expect_peek(&TokenType::LBRACE) {
            panic!("INVALID 3");
        }

        let consequence = self.parse_block_statement();

        let alternative: Option<Box<dyn Node>>;
        if self.peek_token_is(&TokenType::ELSE) {
            self.next_token();
            if !self.expect_peek(&TokenType::LBRACE) {
                panic!("INVALID!!!!");
            }
            alternative = Some(self.parse_block_statement());
        } else {
            alternative = None;
        }

        return Box::new(IfExpression {
            token: og_token,
            condition,
            consequence,
            alternative,
        });
    }

    fn parse_function_expression(&mut self) -> Box<dyn Node> {
        let og_token = self.current_token.clone();

        if !self.expect_peek(&TokenType::LPAREN) {
            panic!("INVALID FUNCTION!");
        }

        let params = self.parse_function_parameters();

        if !self.expect_peek(&TokenType::LBRACE) {
            panic!("INVALID FUNCTION!");
        }

        let body = self.parse_block_statement();

        return Box::new(FunctionLiteralExpression {
            token: og_token,
            parameters: params,
            body,
        });
    }

    fn parse_function_parameters(&mut self) -> Vec<IdentifierExpression> {
        let mut identifiers = vec![];
        if self.peek_token_is(&TokenType::RPAREN) {
            self.next_token();
            return identifiers;
        }

        self.next_token();

        let ident = IdentifierExpression {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone().unwrap(),
        };

        identifiers.push(ident);

        while self.peek_token_is(&TokenType::COMMA) {
            self.next_token();
            self.next_token();
            identifiers.push(IdentifierExpression {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone().unwrap(),
            });
        }

        if !self.expect_peek(&TokenType::RPAREN) {
            panic!("INVALID Function");
        }

        return identifiers;
    }

    fn parse_call_expression(&mut self, func: Box<dyn Node>) -> Box<dyn Node> {
        let og_token = self.current_token.clone();
        let arguments = self.parse_call_arguments();

        return Box::new(CallExpression {
            token: og_token,
            function: func,
            arguments,
        });
    }

    fn parse_call_arguments(&mut self) -> Vec<Box<dyn Node>> {
        let mut args = vec![];

        if self.peek_token_is(&TokenType::RPAREN) {
            self.next_token();
            return args;
        }

        self.next_token();
        args.push(self.parse_expression(PrecedenceType::LOWEST));

        while self.peek_token_is(&TokenType::COMMA) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(PrecedenceType::LOWEST));
        }

        if !self.expect_peek(&TokenType::RPAREN) {
            panic!("INVALID CALL ARGUMENT");
        }

        return args;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let test_string = r#"let x = 5;
        let y = 10;
        let foobar = 838383;"#;

        let lexer = Lexer::new(test_string.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse();

        assert!(
            program.statements.len() == 3,
            "Statements Returned Does not Equal 3"
        );

        let test_literals = vec!["x", "y", "foobar"];
        for i in 0..test_literals.len() {
            test_let_statement(
                program.statements[i]
                    .downcast_ref::<LetStatement>()
                    .unwrap(),
                test_literals[i].to_string(),
            );
        }

        assert_eq!(parser.errors.len(), 0);
    }

    fn test_let_statement(statement: &LetStatement, name: String) {
        // If statement does not equal let
        if statement.token_literal().unwrap() != "let" {
            panic!(
                "Token Literal: {} != 'let'",
                statement.token_literal().unwrap()
            );
        }

        if statement.name.value != name {
            panic!("Name: {} != '{}'", statement.name.value, name);
        }

        if statement.name.token_literal().unwrap() != name {
            panic!(
                "Name: {} != '{}'",
                statement.name.token_literal().unwrap(),
                name
            );
        }
    }

    #[test]
    fn test_return_statements() {
        let test_string = r#"return 5;
        return 10;
        return 993322;"#;

        let lexer = Lexer::new(test_string.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);

        for statement in program.statements {
            assert_eq!(statement.token_literal(), Some("return".to_string()));
        }
    }

    #[test]
    fn test_integer_literal_statements() {
        let test_input = "5; 6;";

        let lexer = Lexer::new(test_input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.statements[0].token_literal().unwrap(), "5");
        assert_eq!(
            program.statements[0]
                .downcast_ref::<ExpressionStatement>()
                .unwrap()
                .expression
                .downcast_ref::<IntegerLiteralExpression>()
                .unwrap()
                .value,
            5
        );
    }

    #[test]
    fn test_prefix_statements() {
        let test_inputs = vec!["!5;", "-15;"];
        for test_input in test_inputs {
            let lexer = Lexer::new(test_input.to_string());
            let mut parser = Parser::new(lexer);

            let program = parser.parse();

            assert_eq!(program.statements.len(), 1);
        }
    }

    #[test]
    fn test_simple_infix_statements() {
        let test_inputs = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for test_input in test_inputs {
            let lexer = Lexer::new(test_input.0.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse();
            assert_eq!(program.statements.len(), 1);

            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<InfixExpression>()
                    .unwrap()
                    .left
                    .token_literal()
                    .unwrap(),
                test_input.1.to_string()
            );
            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<InfixExpression>()
                    .unwrap()
                    .operator,
                test_input.2
            );

            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<InfixExpression>()
                    .unwrap()
                    .right
                    .token_literal()
                    .unwrap(),
                test_input.3.to_string()
            );
        }
    }

    #[test]
    fn test_complex_infix_statements() {
        let test_inputs = vec![
            ("-a + b;", "((-a) + b)"),
            ("!-a;", "(!(-a))"),
            ("a + b + c;", "((a + b) + c)"),
            ("a + b - c;", "((a + b) - c)"),
            ("a * b * c;", "((a * b) * c)"),
            ("a + b / c;", "(a + (b / c))"),
            ("a + b * c + d / e - f;", "(((a + (b * c)) + (d / e)) - f)"),
            ("5 > 4 == 3 < 4;", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4;", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5;",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5;",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];

        for test_input in test_inputs {
            let lexer = Lexer::new(test_input.0.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse();
            assert_eq!(program.statements.len(), 1);

            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .to_string(),
                test_input.1
            );
        }
    }

    #[test]
    fn test_boolean_statements() {
        let test_inputs = vec![
            ("true", "true"),
            ("false;", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true;", "((3 < 5) == true)"),
        ];

        for test_input in test_inputs {
            let lexer = Lexer::new(test_input.0.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse();
            assert_eq!(program.statements.len(), 1);

            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .to_string(),
                test_input.1
            );
        }
    }

    #[test]
    fn test_grouped_statements() {
        let test_inputs = vec![
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2;", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];

        for test_input in test_inputs {
            let lexer = Lexer::new(test_input.0.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse();
            assert_eq!(program.statements.len(), 1);

            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .to_string(),
                test_input.1
            );
        }
    }

    #[test]
    fn test_if_statements() {
        let test_inputs = vec![("if (x < y) { x; }", "(x < y)", "x;")];
        for test_input in test_inputs {
            let lexer = Lexer::new(test_input.0.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse();
            assert_eq!(program.statements.len(), 1);

            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<IfExpression>()
                    .unwrap()
                    .condition
                    .to_string(),
                test_input.1
            );
            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<IfExpression>()
                    .unwrap()
                    .consequence
                    .to_string(),
                test_input.2
            );

            assert!(program.statements[0]
                .downcast_ref::<ExpressionStatement>()
                .unwrap()
                .expression
                .downcast_ref::<IfExpression>()
                .unwrap()
                .alternative
                .is_none());
        }
    }

    #[test]
    fn test_if_else_statement() {
        let test_inputs = vec![(
            "if (x < y) { x; } else { y + 5; }",
            "(x < y)",
            "x;",
            "(y + 5);",
        )];
        for test_input in test_inputs {
            let lexer = Lexer::new(test_input.0.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<IfExpression>()
                    .unwrap()
                    .condition
                    .to_string(),
                test_input.1
            );

            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<IfExpression>()
                    .unwrap()
                    .consequence
                    .to_string(),
                test_input.2
            );

            assert!(program.statements[0]
                .downcast_ref::<ExpressionStatement>()
                .unwrap()
                .expression
                .downcast_ref::<IfExpression>()
                .unwrap()
                .alternative
                .is_some());

            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<IfExpression>()
                    .unwrap()
                    .alternative
                    .as_ref()
                    .unwrap()
                    .to_string(),
                test_input.3
            );
        }
    }

    #[test]
    fn test_function_literal() {
        let test_inputs = vec![
            ("fn(x, y) { x + y; }", "fn(x, y) { (x + y); }", 1, 2, 1),
            (
                "fn() { let a = 3; a + 4; }",
                "fn() { let a = 3; (a + 4); }",
                1,
                0,
                2,
            ),
        ];
        for test_input in test_inputs {
            let lexer = Lexer::new(test_input.0.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse();
            assert_eq!(program.statements.len(), test_input.2);
            assert_eq!(program.statements[0].to_string(), test_input.1);
            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<FunctionLiteralExpression>()
                    .unwrap()
                    .parameters
                    .len(),
                test_input.3
            );

            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<FunctionLiteralExpression>()
                    .unwrap()
                    .body
                    .downcast_ref::<BlockStatement>()
                    .unwrap()
                    .statements
                    .len(),
                test_input.4
            );
        }
    }

    #[test]
    fn test_call_expression() {
        let test_inputs = vec![
            ("add(1, 2, 3)", "add(1, 2, 3)", 1, 3),
            (
                "product(1,2*3, 4+5); run()",
                "product(1, (2 * 3), (4 + 5))",
                2,
                3,
            ),
            ("run()", "run()", 1, 0),
        ];
        for test_input in test_inputs {
            let lexer = Lexer::new(test_input.0.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse();
            assert_eq!(program.statements.len(), test_input.2);
            assert_eq!(program.statements[0].to_string(), test_input.1);
            assert_eq!(
                program.statements[0]
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .downcast_ref::<CallExpression>()
                    .unwrap()
                    .arguments
                    .len(),
                test_input.3
            );
        }
    }

    #[test]
    fn test_eval_integer_expression() {
        let test_inputs = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5", 10),
            ("5 - 5", 0),
            ("5 * 5", 25),
            ("5 / 5", 1),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
            ("5; return 10; 15;", 10),
            ("return 15; 19 + 15; 5 == 5;", 15),
            ("10 == 10; 10 != 11; return 1;", 1),
            ("let a = 10; a;", 10),
        ];
        for test_input in test_inputs {
            test_eval_integer(test_input);
        }
    }

    fn test_eval_integer(test_input: (&str, i64)) {
        let lexer = Lexer::new(test_input.0.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let obj = program.eval();
        assert!(obj.is_some());
        let unwrapped = obj.unwrap();
        assert_eq!(&unwrapped.type_(), &Type::INTEGER);
        assert_eq!(
            &unwrapped.downcast_ref::<Integer>().unwrap().value,
            &test_input.1
        );
    }

    #[test]
    fn test_eval_boolean_expression() {
        let test_inputs = vec![
            ("true", true),
            ("false", false),
            ("!true", false),
            ("!false", true),
            ("5 > 5", false),
            ("5 < 5", false),
            ("5 > 4", true),
            ("5 > 6", false),
            ("5 < 3", false),
            ("1 < 10", true),
            ("1 == 15", false),
            ("1 == 1", true),
            ("1 != 15", true),
            ("1 != 1", false),
            ("(2 > 1) == true", true),
            ("(2 == 2) == true", true),
            ("(2 < 1) == false", true),
            ("let b = true; b;", true),
        ];
        for test_input in test_inputs {
            test_eval_boolean(test_input);
        }
    }

    fn test_eval_boolean(test_input: (&str, bool)) {
        let lexer = Lexer::new(test_input.0.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let obj = program.eval();
        assert!(obj.is_some());
        let unwrapped = obj.unwrap();
        assert_eq!(&unwrapped.type_(), &Type::BOOLEAN);
        assert_eq!(
            &unwrapped.downcast_ref::<Boolean>().unwrap().value,
            &test_input.1
        );
    }

    #[test]
    fn test_if_expression_integer() {
        let test_inputs = vec![
            ("if (5 == 5) { 10; }", 10),
            ("if (1 == 2) { 10; } else { 5; }", 5),
        ];
        for test_input in test_inputs {
            test_eval_integer(test_input);
        }
    }

    #[test]
    fn test_error_handling() {
        let test_inputs = vec![
            ("5 + true", "type mismatch: INTEGER + BOOLEAN"),
            ("true - 10", "type mismatch: BOOLEAN - INTEGER"),
            (
                "10; 5 + true; return 15;",
                "type mismatch: INTEGER + BOOLEAN",
            ),
            ("-true", "unknown operator: -BOOLEAN"),
            ("-(5 + true)", "type mismatch: INTEGER + BOOLEAN"),
            ("if (5 + true) { x }", "type mismatch: INTEGER + BOOLEAN"),
            ("foobar;", "identifier not found: foobar"),
        ];

        for test_input in test_inputs {
            test_eval_error(test_input);
        }
    }

    fn test_eval_error(test_input: (&str, &str)) {
        let lexer = Lexer::new(test_input.0.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let obj = program.eval();
        assert!(obj.is_some());
        let unwrapped = obj.unwrap();

        assert_eq!(unwrapped.inspect(), test_input.1);
    }
}
