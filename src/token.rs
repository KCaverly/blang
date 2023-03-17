#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

pub struct Token {
    pub token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        return Token {
            token_type,
            literal,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
