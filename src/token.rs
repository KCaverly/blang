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

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<&str>) -> Token {
        let lit: String;
        if literal.is_some() {
            lit = literal.unwrap().to_string();
            return Token {
                token_type,
                literal: Some(lit),
            };
        } else {
            return Token {
                token_type,
                literal: None,
            };
        }
    }
}

#[cfg(test)]
mod tests {}
