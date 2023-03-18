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
    pub literal: Option<char>,
}

impl Token {
    pub const fn new(token_type: TokenType, literal: Option<char>) -> Token {
        return Token {
            token_type,
            literal,
        };
    }
}

pub const TOKENS: [Token; 9] = [
    Token::new(TokenType::ASSIGN, Some('=')),
    Token::new(TokenType::PLUS, Some('+')),
    Token::new(TokenType::LPAREN, Some('(')),
    Token::new(TokenType::RPAREN, Some(')')),
    Token::new(TokenType::LBRACE, Some('{')),
    Token::new(TokenType::RBRACE, Some('}')),
    Token::new(TokenType::COMMA, Some(',')),
    Token::new(TokenType::SEMICOLON, Some(';')),
    Token::new(TokenType::EOF, None),
];
