extern crate downcast_rs;

use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use downcast_rs::{impl_downcast, Downcast};

trait Node {
    fn token_literal(&self) -> Option<String>;
    fn to_string(&self) -> String;
}

trait Statement: Downcast {
    fn token_literal(&self) -> Option<String>;
    fn to_string(&self) -> String;
}

impl_downcast!(Statement);

trait Expression: Downcast {
    fn token_literal(&self) -> Option<String>;
    fn to_string(&self) -> String;
}

impl_downcast!(Expression);

/////////////
// Program //
/////////////

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

///////////////
// Statement //
///////////////

struct LetStatement {
    token: Token,
    name: IdentifierExpression,
    value: Box<dyn Expression>,
}

impl Statement for LetStatement {
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
}

struct ReturnStatement {
    token: Token,
    value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }

    fn to_string(&self) -> String {
        return format!(
            "{} {}",
            self.token_literal().unwrap(),
            self.value.to_string()
        );
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
    fn to_string(&self) -> String {
        return self.expression.to_string();
    }
}

////////////////
// Expression //
////////////////

struct IdentifierExpression {
    token: Token,
    value: String,
}

impl Expression for IdentifierExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        return self.value.clone();
    }
}

struct IntegerLiteralExpression {
    token: Token,
    value: usize,
}

impl Expression for IntegerLiteralExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        return self.value.clone().to_string();
    }
}

struct PrefixExpression {
    token: Token,
    operator: String,
    right: Box<dyn Expression>,
}

impl Expression for PrefixExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        return format!("({}{})", self.operator, self.right.to_string());
    }
}

////////////
// Parser //
////////////

struct Parser {
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

    pub fn parse(&mut self) -> Program {
        // Create Blank Program to Start
        let mut program = Program { statements: vec![] };

        // Iterate through all token in the Lexer
        // TODO: We have to handle semicolons at some point
        while !self.current_token_is(&TokenType::EOF)
            & !self.current_token_is(&TokenType::SEMICOLON)
        {
            let statement = self.parse_statement();
            program.statements.push(statement);

            self.next_token();
        }

        return program;
    }

    fn parse_statement(&mut self) -> Box<dyn Statement> {
        let token_type = self.current_token.token_type;
        let statement = match token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            TokenType::INT => self.parse_expression_statement(),
            TokenType::BANG => self.parse_expression_statement(),
            TokenType::MINUS => self.parse_expression_statement(),
            _ => panic!("PANIC!"),
        };

        return statement;
    }

    fn parse_let_statement(&mut self) -> Box<dyn Statement> {
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
        }

        while !self.current_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        // TODO: For now this value returned is a dummy identifier.
        // TODO: This will have to be updated at some future point
        return Box::new(LetStatement {
            token: og_token,
            name,
            value: Box::new(IdentifierExpression {
                token: self.peek_token.clone(),
                value: "".to_string(),
            }),
        });
    }
    fn parse_return_statement(&mut self) -> Box<dyn Statement> {
        let og_token = self.current_token.clone();
        self.next_token();

        // TODO: Again this is not correctly parsing the value
        while !self.current_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        return Box::new(ReturnStatement {
            token: og_token,
            value: Box::new(IdentifierExpression {
                token: self.peek_token.clone(),
                value: "".to_string(),
            }),
        });
    }
    // fn parse_integer_statement(&mut self) -> Box<dyn Statement> {
    //     return Box::new(IntegerLiteralStatement {
    //         token: self.current_token.clone(),
    //         value: self
    //             .current_token
    //             .clone()
    //             .literal
    //             .unwrap()
    //             .parse::<usize>()
    //             .unwrap(),
    //     });
    // }
    fn parse_expression_statement(&mut self) -> Box<dyn Statement> {
        let expr = self.parse_expression();
        return Box::new(ExpressionStatement {
            token: self.current_token.clone(),
            expression: expr,
        });
    }

    fn parse_expression(&mut self) -> Box<dyn Expression> {
        let token_type = self.current_token.token_type;
        println!("{:?}", token_type);
        let expr = match token_type {
            TokenType::INT => self.parse_integer_expression(),
            TokenType::BANG => self.parse_prefix_expression(),
            TokenType::MINUS => self.parse_prefix_expression(),

            _ => panic!("PANICKING!"),
        };
        return expr;
    }

    fn parse_integer_expression(&mut self) -> Box<dyn Expression> {
        return Box::new(IntegerLiteralExpression {
            token: self.current_token.clone(),
            value: self
                .current_token
                .clone()
                .literal
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        });
    }
    fn parse_prefix_expression(&mut self) -> Box<dyn Expression> {
        let og_token = self.current_token.clone();
        self.next_token();

        return Box::new(PrefixExpression {
            token: og_token.clone(),
            operator: og_token.literal.clone().unwrap(),
            right: self.parse_expression(),
        });
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
        let test_input = "5;";

        let lexer = Lexer::new(test_input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse();

        assert_eq!(program.statements.len(), 1);
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
}
