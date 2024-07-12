use core::fmt;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum TokenType {
    // single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // one or two character tokens
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    // literals
    Identifire(String), String(String), Number(f64),

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Var, While, 

    Eof,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Option<String>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lexeme_value = match &self.lexeme {
            Some(value) => value,
            None => "",
        };
        write!(f, "{:?} {} {}", self.token_type, lexeme_value, self.line)
    }
}