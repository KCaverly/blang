use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        return Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Some(' '),
        };
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            return None;
        } else {
            return Some(self.input[self.read_position]);
        }
    }

    fn match_char(&mut self) -> Option<Token> {
        let token = match self.ch {
            // Math Operators
            Some('=') => {
                let peeked = self.peek_char();

                if peeked.is_none() {
                    Some(Token::new(TokenType::ASSIGN, Some("=")))
                } else if peeked.unwrap() == '=' {
                    self.read_char();
                    Some(Token::new(TokenType::EQ, Some("==")))
                } else {
                    Some(Token::new(TokenType::ASSIGN, Some("=")))
                }
            }

            Some('!') => {
                let peeked = self.peek_char();
                if peeked.is_none() {
                    Some(Token::new(TokenType::BANG, Some("!")))
                } else if peeked.unwrap() == '=' {
                    self.read_char();
                    Some(Token::new(TokenType::NEQ, Some("!=")))
                } else {
                    Some(Token::new(TokenType::BANG, Some("!")))
                }
            }

            Some('+') => Some(Token::new(TokenType::PLUS, Some("+").as_deref())),
            Some('/') => Some(Token::new(TokenType::SLASH, Some("/"))),
            Some('*') => Some(Token::new(TokenType::ASTERISK, Some("*"))),
            Some('-') => Some(Token::new(TokenType::MINUS, Some("-"))),
            Some('>') => Some(Token::new(TokenType::GT, Some(">"))),
            Some('<') => Some(Token::new(TokenType::LT, Some("<"))),

            // Groupings
            Some('(') => Some(Token::new(
                TokenType::LPAREN,
                Some(self.ch.unwrap().to_string()).as_deref(),
            )),
            Some(')') => Some(Token::new(
                TokenType::RPAREN,
                Some(self.ch.unwrap().to_string()).as_deref(),
            )),
            Some('{') => Some(Token::new(
                TokenType::LBRACE,
                Some(self.ch.unwrap().to_string()).as_deref(),
            )),
            Some('}') => Some(Token::new(
                TokenType::RBRACE,
                Some(self.ch.unwrap().to_string()).as_deref(),
            )),

            // Flow
            Some(',') => Some(Token::new(
                TokenType::COMMA,
                Some(self.ch.unwrap().to_string()).as_deref(),
            )),

            Some(';') => Some(Token::new(
                TokenType::SEMICOLON,
                Some(self.ch.unwrap().to_string()).as_deref(),
            )),

            _ => None,
        };

        return token;
    }

    fn match_alphabetic_span(&mut self) -> Option<Token> {
        let mut ident: Vec<char> = Vec::new();
        if self.ch.is_none() {
            return None;
        }
        while self.ch.unwrap().is_alphabetic() & !self.ch.unwrap().is_whitespace() {
            ident.push(self.ch.unwrap());
            self.read_char();
        }

        let ident_string: String = ident.iter().collect();

        if ident_string.len() == 0 {
            return None;
        }

        let token = match &*ident_string {
            "let" => Some(Token::new(TokenType::LET, Some("let"))),
            "fn" => Some(Token::new(TokenType::FUNCTION, Some("fn"))),
            "if" => Some(Token::new(TokenType::IF, Some("if"))),
            "else" => Some(Token::new(TokenType::ELSE, Some("else"))),
            "return" => Some(Token::new(TokenType::RETURN, Some("return"))),
            "true" => Some(Token::new(TokenType::TRUE, Some("true"))),
            "false" => Some(Token::new(TokenType::FALSE, Some("false"))),
            _ => Some(Token::new(TokenType::IDENT, Some(&*ident_string))),
        };

        self.read_position -= 1;
        self.position -= 1;

        return token;
    }

    fn match_numeric_span(&mut self) -> Option<Token> {
        let mut numeric: Vec<char> = Vec::new();
        if self.ch.is_none() {
            return None;
        }
        while self.ch.unwrap().is_numeric() {
            numeric.push(self.ch.unwrap());
            self.read_char();
        }

        let numeric_string: String = numeric.iter().collect();
        if numeric_string.len() == 0 {
            return None;
        }

        self.read_position -= 1;
        self.position -= 1;

        return Some(Token::new(TokenType::INT, Some(&*numeric_string)));
    }

    pub fn next_token(&mut self) -> Token {
        // Next Token:
        // 1. Match Char
        //      - Identify if character is special character
        // 2. Match Keyword
        //      - Identify if character span is keyword
        // 3. Match Alphanumeric span to Identifier
        //      - Set character span to identifier
        // 4. If no match, set to ILLEGAL

        let mut token: Option<Token>;

        self.read_char();
        if self.ch.is_none() {
            return Token::new(TokenType::EOF, None);
        } else if self.ch.unwrap().is_whitespace() {
            return self.next_token();
        }

        token = self.match_char();
        if token.is_some() {
            return token.unwrap();
        }

        token = self.match_alphabetic_span();
        if token.is_some() {
            return token.unwrap();
        }

        token = self.match_numeric_span();
        if token.is_some() {
            return token.unwrap();
        }

        return Token::new(TokenType::ILLEGAL, None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_lexer() {
        let test_string = "=+(){},;".to_string();

        let test_tokens: Vec<Token> = vec![
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::PLUS, Some("+")),
            Token::new(TokenType::LPAREN, Some("(")),
            Token::new(TokenType::RPAREN, Some(")")),
            Token::new(TokenType::LBRACE, Some("{")),
            Token::new(TokenType::RBRACE, Some("}")),
            Token::new(TokenType::COMMA, Some(",")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::EOF, None),
        ];

        let mut lexer = Lexer::new(test_string);

        for test_token in test_tokens {
            let token = lexer.next_token();
            assert_eq!(token, test_token);
        }
    }

    #[test]
    fn test_multiline_lexer() {
        let test_string = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);"#
            .to_string();

        let test_tokens: Vec<Token> = vec![
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("five")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::INT, Some("5")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("ten")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::INT, Some("10")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("add")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::FUNCTION, Some("fn")),
            Token::new(TokenType::LPAREN, Some("(")),
            Token::new(TokenType::IDENT, Some("x")),
            Token::new(TokenType::COMMA, Some(",")),
            Token::new(TokenType::IDENT, Some("y")),
            Token::new(TokenType::RPAREN, Some(")")),
            Token::new(TokenType::LBRACE, Some("{")),
            Token::new(TokenType::IDENT, Some("x")),
            Token::new(TokenType::PLUS, Some("+")),
            Token::new(TokenType::IDENT, Some("y")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::RBRACE, Some("}")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("result")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::IDENT, Some("add")),
            Token::new(TokenType::LPAREN, Some("(")),
            Token::new(TokenType::IDENT, Some("five")),
            Token::new(TokenType::COMMA, Some(",")),
            Token::new(TokenType::IDENT, Some("ten")),
            Token::new(TokenType::RPAREN, Some(")")),
            Token::new(TokenType::SEMICOLON, Some(";")),
        ];

        let mut lexer = Lexer::new(test_string);
        for test_token in test_tokens {
            let token = lexer.next_token();
            assert_eq!(token, test_token);
        }
    }

    #[test]
    fn test_extended_lexer() {
        let test_string = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };
        
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        
        "#;

        let test_tokens = vec![
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("five")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::INT, Some("5")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("ten")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::INT, Some("10")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("add")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::FUNCTION, Some("fn")),
            Token::new(TokenType::LPAREN, Some("(")),
            Token::new(TokenType::IDENT, Some("x")),
            Token::new(TokenType::COMMA, Some(",")),
            Token::new(TokenType::IDENT, Some("y")),
            Token::new(TokenType::RPAREN, Some(")")),
            Token::new(TokenType::LBRACE, Some("{")),
            Token::new(TokenType::IDENT, Some("x")),
            Token::new(TokenType::PLUS, Some("+")),
            Token::new(TokenType::IDENT, Some("y")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::RBRACE, Some("}")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("result")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::IDENT, Some("add")),
            Token::new(TokenType::LPAREN, Some("(")),
            Token::new(TokenType::IDENT, Some("five")),
            Token::new(TokenType::COMMA, Some(",")),
            Token::new(TokenType::IDENT, Some("ten")),
            Token::new(TokenType::RPAREN, Some(")")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::BANG, Some("!")),
            Token::new(TokenType::MINUS, Some("-")),
            Token::new(TokenType::SLASH, Some("/")),
            Token::new(TokenType::ASTERISK, Some("*")),
            Token::new(TokenType::INT, Some("5")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::INT, Some("5")),
            Token::new(TokenType::LT, Some("<")),
            Token::new(TokenType::INT, Some("10")),
            Token::new(TokenType::GT, Some(">")),
            Token::new(TokenType::INT, Some("5")),
            Token::new(TokenType::SEMICOLON, Some(";")),
        ];

        let mut lexer = Lexer::new(test_string.to_string());
        for test_token in test_tokens {
            let token = lexer.next_token();
            assert_eq!(token, test_token);
        }
    }

    #[test]
    fn test_if_else_lexer() {
        let test_string = r#"let x = 5;
        let y = 10;

        if (y > x) {
            return true;
        } else {
            return false;
        }
        "#;

        let test_tokens = vec![
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("x")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::INT, Some("5")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("y")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::INT, Some("10")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::IF, Some("if")),
            Token::new(TokenType::LPAREN, Some("(")),
            Token::new(TokenType::IDENT, Some("y")),
            Token::new(TokenType::GT, Some(">")),
            Token::new(TokenType::IDENT, Some("x")),
            Token::new(TokenType::RPAREN, Some(")")),
            Token::new(TokenType::LBRACE, Some("{")),
            Token::new(TokenType::RETURN, Some("return")),
            Token::new(TokenType::TRUE, Some("true")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::RBRACE, Some("}")),
            Token::new(TokenType::ELSE, Some("else")),
            Token::new(TokenType::LBRACE, Some("{")),
            Token::new(TokenType::RETURN, Some("return")),
            Token::new(TokenType::FALSE, Some("false")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::RBRACE, Some("}")),
        ];
        let mut lexer = Lexer::new(test_string.to_string());
        for test_token in test_tokens {
            let token = lexer.next_token();
            assert_eq!(token, test_token);
        }
    }

    #[test]
    fn test_eq_neq_lexer() {
        let test_string = r#"
            let x = 5;
            10 == 15;
            12 != 9;
            "#;

        let test_tokens = vec![
            Token::new(TokenType::LET, Some("let")),
            Token::new(TokenType::IDENT, Some("x")),
            Token::new(TokenType::ASSIGN, Some("=")),
            Token::new(TokenType::INT, Some("5")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::INT, Some("10")),
            Token::new(TokenType::EQ, Some("==")),
            Token::new(TokenType::INT, Some("15")),
            Token::new(TokenType::SEMICOLON, Some(";")),
            Token::new(TokenType::INT, Some("12")),
            Token::new(TokenType::NEQ, Some("!=")),
            Token::new(TokenType::INT, Some("9")),
            Token::new(TokenType::SEMICOLON, Some(";")),
        ];

        let mut lexer = Lexer::new(test_string.to_string());
        for test_token in test_tokens {
            let token = lexer.next_token();
            assert_eq!(token, test_token);
        }
    }
}
