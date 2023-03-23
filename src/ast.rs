extern crate downcast_rs;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use downcast_rs::{impl_downcast, Downcast};

trait Node {
    fn token_literal(&self) -> Option<String>;
}

trait Statement: Downcast {
    fn token_literal(&self) -> Option<String>;
    fn statement_node(&self);
}

impl_downcast!(Statement);

trait Expression {
    fn token_literal(&self) -> Option<String>;
    fn expression_node(&self);
}

struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    fn token_literal(&self) -> Option<String> {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return None;
        }
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
}

struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            current_token: None,
            peek_token: None,
        };

        p.next_token();
        p.next_token();

        return p;
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse(&mut self) -> Program {
        println!("Current Token: {:?}", self.current_token);
        println!("Peek Token: {:?}", self.peek_token);

        // Create a Blank Program to Start
        let mut program = Program { statements: vec![] };

        // Iterate through all tokens in the Lexer
        while self.current_token.clone().unwrap().token_type != TokenType::EOF {
            let statement = self.parse_statement();
            program.statements.push(statement);

            self.next_token();
        }

        return program;
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

    fn parse_statement(&mut self) -> Box<dyn Statement> {
        let token_type = self.current_token.clone().unwrap().token_type;
        let statement = match token_type {
            TokenType::LET => self.parse_let_statement(),
            _ => panic!("Not PARSED!"),
        };

        let unwrapped = statement.unwrap();

        println!("Statement: {:?}", &unwrapped.token_literal());

        return unwrapped;
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let unwrapped_cur = self.current_token.clone().unwrap();

        if self.peek_token_is(TokenType::ASSIGN) {
            return None;
        } else {
            self.next_token();
        }

        let name = Identifier {
            token: self.current_token.clone().unwrap(),
            value: self.current_token.clone().unwrap().literal.unwrap(),
        };

        while !self.current_token_is(TokenType::SEMICOLON) {
            println!("{:?}", &self.current_token);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parser() {
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
            println!("{}", i);
            test_let_statement(
                program.statements[i]
                    .downcast_ref::<LetStatement>()
                    .unwrap(),
                test_literals[i].to_string(),
            );
        }
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
}
