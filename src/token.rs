#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,
    MINUS,
    SLASH,
    ASTERISK,
    LT,
    GT,
    BANG,

    EQ,
    NEQ,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    IF,
    ELSE,
    RETURN,

    TRUE,
    FALSE,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
