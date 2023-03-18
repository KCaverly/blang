use crate::token::{Token, TokenType, TOKENS};

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
            ch: Some('/'),
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

    pub fn next_token(&mut self) -> TokenType {
        self.read_char();
        if self.ch.is_none() {
            return TokenType::EOF;
        } else {
            for t in TOKENS {
                if t.literal == self.ch {
                    return t.token_type;
                }
            }
        }

        return TokenType::ILLEGAL;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let test_string = "=+(){},;".to_string();

        let mut test_tokens: Vec<TokenType> = Vec::new();
        test_tokens.push(TokenType::ASSIGN);
        test_tokens.push(TokenType::PLUS);
        test_tokens.push(TokenType::LPAREN);
        test_tokens.push(TokenType::RPAREN);
        test_tokens.push(TokenType::LBRACE);
        test_tokens.push(TokenType::RBRACE);
        test_tokens.push(TokenType::COMMA);
        test_tokens.push(TokenType::SEMICOLON);
        test_tokens.push(TokenType::EOF);

        let mut lexer = Lexer::new(test_string);

        for test_token in test_tokens {
            let token = lexer.next_token();
            assert_eq!(token, test_token);
        }
    }
}
