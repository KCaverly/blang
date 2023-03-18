use crate::token::{Token, TokenType};

struct Lexer {
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
            ch: None,
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

    pub fn read_span(&mut self) -> String {
        let mut ident: Vec<char> = Vec::new();
        while self.ch.unwrap().is_alphanumeric() {
            println!("{}", self.ch.unwrap());
            ident.push(self.ch.unwrap());
            self.read_char();
        }

        let ident_string: String = ident.iter().collect();

        return ident_string;
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

        self.read_char();

        let token = match self.ch {
            // EOF
            None => Token::new(TokenType::EOF, None),

            // Math Operators
            Some('=') => Token::new(
                TokenType::ASSIGN,
                Some(self.ch.unwrap().to_string()).as_deref(),
            ),
            Some('+') => Token::new(
                TokenType::PLUS,
                Some(self.ch.unwrap().to_string()).as_deref(),
            ),

            // Groupings
            Some('(') => Token::new(
                TokenType::LPAREN,
                Some(self.ch.unwrap().to_string()).as_deref(),
            ),
            Some(')') => Token::new(
                TokenType::RPAREN,
                Some(self.ch.unwrap().to_string()).as_deref(),
            ),
            Some('{') => Token::new(
                TokenType::LBRACE,
                Some(self.ch.unwrap().to_string()).as_deref(),
            ),
            Some('}') => Token::new(
                TokenType::RBRACE,
                Some(self.ch.unwrap().to_string()).as_deref(),
            ),

            // Flow
            Some(',') => Token::new(
                TokenType::COMMA,
                Some(self.ch.unwrap().to_string()).as_deref(),
            ),

            Some(';') => Token::new(
                TokenType::SEMICOLON,
                Some(self.ch.unwrap().to_string()).as_deref(),
            ),

            _ => {
                let span = &*self.read_span();
                let tkn = match span {
                    "let" => Token::new(TokenType::LET, Some(span)),
                    "fn" => Token::new(TokenType::FUNCTION, Some(span)),
                    _ => Token::new(TokenType::IDENT, Some(span)),
                };
                tkn
            }
        };

        return token;
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
}
