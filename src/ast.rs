use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

trait Node {
    fn token_literal(&self) -> Option<String>;
}

trait Statement {
    fn token_literal(&self) -> Option<String>;
    fn statement_node(&self);
}

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

    fn parse_statement(&self) -> Box<dyn Statement> {
        return Box::new(self.parse_let_statement());
    }

    fn parse_let_statement(&self) -> LetStatement {
        return LetStatement {
            token: self.current_token.clone().unwrap(),
            name: Identifier {
                token: self.peek_token.clone().unwrap(),
                value: "".to_string(),
            },
            value: Box::new(Identifier {
                token: self.peek_token.clone().unwrap(),
                value: "".to_string(),
            }),
        };
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

        for statement in &program.statements {
            println!("{:?}", statement.token_literal());
        }

        assert!(
            program.statements.len() == 3,
            "Statements Returned Does not Equal 3"
        );

        panic!("HOW IS THIS WORKING!");
    }
}
