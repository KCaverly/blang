extern crate downcast_rs;
use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use downcast_rs::{impl_downcast, Downcast};

trait Node {
    fn token_literal(&self) -> Option<String>;
    fn to_string(&self) -> String;
}

trait Statement: Downcast {
    fn token_literal(&self) -> Option<String>;
    fn statement_node(&self);
    fn to_string(&self) -> String;
}

impl_downcast!(Statement);

trait Expression {
    fn token_literal(&self) -> Option<String>;
    fn expression_node(&self);
    fn to_string(&self) -> String;
}

struct Program {
    statements: Vec<Box<dyn Statement>>,
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
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }

    fn statement_node(&self) {}

    fn to_string(&self) -> String {
        return format!(
            "{} {} = {}",
            self.token_literal().unwrap(),
            self.name.to_string(),
            self.value.to_string()
        )
        .to_string();
    }
}

struct ReturnStatement {
    token: Token,
    value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }

    fn statement_node(&self) {}
    fn to_string(&self) -> String {
        return format!(
            "{} {}",
            self.token_literal().unwrap(),
            self.value.to_string()
        );
    }
}

struct Identifier {
    token: Token,
    value: String,
}

impl Expression for Identifier {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn expression_node(&self) {}
    fn to_string(&self) -> String {
        return self.value.clone();
    }
}

struct ExpressionStatement {
    token: Token,
    expression: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn statement_node(&self) {}
    fn to_string(&self) -> String {
        return self.expression.to_string();
    }
}

enum Precedence {
    LOWEST = 1,
    EQUALS = 2,
    LESSGREATER = 3,
    SUM = 4,
    PRODUCT = 5,
    PREFIX = 6,
    CALL = 7,
}

struct Parser<'a> {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, &'a dyn Fn() -> Box<dyn Expression>>,
    infix_parse_fns: HashMap<TokenType, &'a dyn Fn(Box<dyn Expression>) -> Box<dyn Expression>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer) -> Parser<'a> {
        let mut p = Parser {
            lexer,
            current_token: None,
            peek_token: None,
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // Load the First Two Tokens
        p.next_token();
        p.next_token();

        return p;
    }

    fn register_prefix(
        &mut self,
        token_type: TokenType,
        function: &'a dyn Fn() -> Box<dyn Expression>,
    ) {
        self.prefix_parse_fns.insert(token_type, function);
    }

    fn register_infix(
        &mut self,
        token_type: TokenType,
        function: &'a dyn Fn(Box<dyn Expression>) -> Box<dyn Expression>,
    ) {
        self.infix_parse_fns.insert(token_type, function);
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse(&mut self) -> Program {
        // Create a Blank Program to Start
        let mut program = Program { statements: vec![] };

        // Iterate through all tokens in the Lexer
        while !self.current_token_is(TokenType::EOF) {
            let statement = self.parse_statement();
            program.statements.push(statement);

            self.next_token();
        }

        return program;
    }

    fn parse_identifier(&self) -> Box<dyn Expression> {
        return Box::new(Identifier {
            token: self.current_token.clone().unwrap(),
            value: self.current_token.clone().unwrap().literal.unwrap(),
        });
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        if self.current_token.clone().unwrap().token_type == token_type {
            return true;
        } else {
            return false;
        }
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        if self.peek_token.clone().unwrap().token_type == token_type {
            return true;
        } else {
            return false;
        }
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        } else {
            let msg = format!(
                "Expected next token to be {:?}, got {:?} instead",
                self.peek_token.clone().unwrap().token_type,
                token_type
            );
            self.errors.push(msg);
            return false;
        }
    }

    fn parse_statement(&mut self) -> Box<dyn Statement> {
        let token_type = self.current_token.clone().unwrap().token_type;
        let statement = match token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };

        let unwrapped = statement.unwrap();

        return unwrapped;
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let unwrapped_cur = self.current_token.clone().unwrap();

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let name = Identifier {
            token: self.current_token.clone().unwrap(),
            value: self.current_token.clone().unwrap().literal.unwrap(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        return Some(Box::new(LetStatement {
            token: unwrapped_cur,
            name,
            value: Box::new(Identifier {
                token: self.peek_token.clone().unwrap(),
                value: "".to_string(),
            }),
        }));
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let og_token = self.current_token.clone();
        self.next_token();

        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        return Some(Box::new(ReturnStatement {
            token: og_token.unwrap(),
            value: Box::new(Identifier {
                token: self.peek_token.clone().unwrap(),
                value: "".to_string(),
            }),
        }));
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let og_token = self.current_token.clone();

        let expr = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        return Some(Box::new(ExpressionStatement {
            token: og_token.unwrap(),
            expression: expr.unwrap(),
        }));
    }

    fn parse_prefix(&self) -> Option<Box<dyn Expression>> {
        let current_token_type = self.current_token.clone().unwrap().token_type;
        match current_token_type {
            TokenType::IDENT => Some(self.parse_identifier()),
            _ => None,
        }
    }

    fn parse_expression(&self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let prefix = self.parse_prefix();

        return prefix;
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
    fn test_string() {
        let test_program = Program {
            statements: vec![Box::new(LetStatement {
                token: Token {
                    token_type: TokenType::LET,
                    literal: Some("let".to_string()),
                },
                name: Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: Some("myVar".to_string()),
                    },
                    value: "myVar".to_string(),
                },
                value: Box::new(Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: Some("anotherVar".to_string()),
                    },
                    value: "anotherVar".to_string(),
                }),
            })],
        };

        // This doesnt include semicolons yet - not sure where those come in
        assert_eq!(test_program.to_string(), "let myVar = anotherVar");
    }

    #[test]
    fn test_identifier_expression() {
        let test_input = "foobar;";

        let lexer = Lexer::new(test_input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse();

        assert_eq!(program.statements.len(), 1);
    }
}
