use crate::token::{Token, TokenType};

struct Lexer {
    input: String,
    position: i64,
    read_position: i64,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        return Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: None,
        };
    }

    pub fn next_token(&self) -> &TokenType {
        return &TokenType::ASSIGN;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let test_string = "=+(){},;".to_string();

        let mut test_tokens: Vec<Token> = Vec::new();
        test_tokens.push(Token::new(TokenType::ASSIGN, "=".to_string()));
        test_tokens.push(Token::new(TokenType::PLUS, "+".to_string()));
        test_tokens.push(Token::new(TokenType::LPAREN, "(".to_string()));
        test_tokens.push(Token::new(TokenType::RPAREN, ")".to_string()));
        test_tokens.push(Token::new(TokenType::LBRACE, "{".to_string()));
        test_tokens.push(Token::new(TokenType::RBRACE, "}".to_string()));
        test_tokens.push(Token::new(TokenType::COMMA, ",".to_string()));
        test_tokens.push(Token::new(TokenType::SEMICOLON, ";".to_string()));
        test_tokens.push(Token::new(TokenType::EOF, "".to_string()));

        let lexer = Lexer::new(test_string);

        for test_token in test_tokens {
            let token = lexer.next_token();

            assert_eq!(token, &test_token.token_type);
        }
    }
}
