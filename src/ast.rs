extern crate downcast_rs;
extern crate lazy_static;

use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use downcast_rs::{impl_downcast, Downcast};
use lazy_static::lazy_static;
use std::collections::HashMap;

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

struct BlockStatement {
    token: Token,
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for BlockStatement {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        let mut str: Vec<String> = Vec::new();
        for statement in &self.statements {
            str.push(statement.to_string());
        }
        return str.join(" ");
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

struct BooleanExpression {
    token: Token,
    value: bool,
}

impl Expression for BooleanExpression {
    fn token_literal(&self) -> Option<String> {
        return self.token.literal.to_owned();
    }
    fn to_string(&self) -> String {
        return self.value.to_string();
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

struct InfixExpression {
    token: Token,
    left: Box<dyn Expression>,
    operator: String,
    right: Box<dyn Expression>,
}

impl Expression for InfixExpression {
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
}

struct IfExpression {
    token: Token,
    condition: Box<dyn Expression>,
    consequence: Box<dyn Statement>,
    alternative: Option<Box<dyn Statement>>,
}

impl Expression for IfExpression {
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
        (TokenType::ASTERISK, PrecedenceType::PRODUCT)
    ]);
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
            TokenType::IDENT => self.parse_expression_statement(),
            TokenType::TRUE => self.parse_expression_statement(),
            TokenType::FALSE => self.parse_expression_statement(),
            TokenType::LPAREN => self.parse_expression_statement(),
            TokenType::IF => self.parse_expression_statement(),
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

    fn parse_expression_statement(&mut self) -> Box<dyn Statement> {
        let expr = self.parse_expression(PrecedenceType::LOWEST);
        return Box::new(ExpressionStatement {
            token: self.current_token.clone(),
            expression: expr,
        });
    }

    fn parse_block_statement(&mut self) -> Box<dyn Statement> {
        let og_token = self.current_token.clone();
        let mut statements = vec![];

        self.next_token();

        while !self.current_token_is(&TokenType::RBRACE) && !self.current_token_is(&TokenType::EOF)
        {
            let statement = self.parse_statement();
            statements.push(statement);
            self.next_token();
        }

        return Box::new(BlockStatement {
            token: og_token,
            statements,
        });
    }

    fn parse_expression(&mut self, precedence: PrecedenceType) -> Box<dyn Expression> {
        let token_type = self.current_token.token_type;

        // Parse Left Side of Expression
        let left_expr = match token_type {
            TokenType::INT => Some(self.parse_integer_expression()),
            TokenType::BANG => Some(self.parse_prefix_expression()),
            TokenType::MINUS => Some(self.parse_prefix_expression()),
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
                    TokenType::PLUS => self.parse_infix_expression(expr),
                    TokenType::MINUS => self.parse_infix_expression(expr),
                    TokenType::SLASH => self.parse_infix_expression(expr),
                    TokenType::ASTERISK => self.parse_infix_expression(expr),
                    TokenType::EQ => self.parse_infix_expression(expr),
                    TokenType::NEQ => self.parse_infix_expression(expr),
                    TokenType::GT => self.parse_infix_expression(expr),
                    TokenType::LT => self.parse_infix_expression(expr),
                    _ => panic!("PANICKING!"),
                };
            }
            return expr;
        }
    }

    fn parse_integer_expression(&mut self) -> Box<dyn Expression> {
        let expr = Box::new(IntegerLiteralExpression {
            token: self.current_token.clone(),
            value: self
                .current_token
                .clone()
                .literal
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        });
        return expr;
    }
    fn parse_identifier_expression(&mut self) -> Box<dyn Expression> {
        let expr = Box::new(IdentifierExpression {
            token: self.current_token.clone(),
            value: self.current_token.clone().literal.unwrap(),
        });
        return expr;
    }

    fn parse_boolean_expression(&mut self) -> Box<dyn Expression> {
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

    fn parse_prefix_expression(&mut self) -> Box<dyn Expression> {
        let og_token = self.current_token.clone();
        self.next_token();

        return Box::new(PrefixExpression {
            token: og_token.clone(),
            operator: og_token.literal.clone().unwrap(),
            right: self.parse_expression(PrecedenceType::PREFIX),
        });
    }
    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Box<dyn Expression> {
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

    fn parse_grouped_expression(&mut self) -> Box<dyn Expression> {
        self.next_token();

        let expr = self.parse_expression(PrecedenceType::LOWEST);
        if !self.expect_peek(&TokenType::RPAREN) {
            panic!("{}", "DOES NOT INCLUDE RPAREN");
        }

        return expr;
    }

    fn parse_if_expression(&mut self) -> Box<dyn Expression> {
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

        let alternative: Option<Box<dyn Statement>>;
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
            ("true;", "true"),
            ("false;", "false"),
            ("3 > 5 == false;", "((3 > 5) == false)"),
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
            ("1 + (2 + 3) + 4;", "((1 + (2 + 3)) + 4)"),
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
        let test_input = "if (x < y) { x }";
        let lexer = Lexer::new(test_input.to_string());
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
            "(x < y)"
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
            "x"
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

    #[test]
    fn test_if_else_statement() {
        let test_input = "if (x < y) { x } else { y + 5 }";
        let lexer = Lexer::new(test_input.to_string());
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
            "(x < y)"
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
            "x"
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
            "(y + 5)"
        );
    }
}
