use std::{error::Error, fmt::{Debug, Display}, str::FromStr};
use std::fmt;
use tokens::{TokenType, Token};

pub mod tokens;

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { 
            source: source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                ScanningResult::Token(token) => {
                    self.add_token(token, &mut tokens)
                },
                ScanningResult::Error(error) => {
                    println!("{}", error);
                    break;
                },
                ScanningResult::NewLine => {
                    self.line += 1
                },
                ScanningResult::Skip => continue
            }
        }

        tokens.push(Token::new(TokenType::Eof, None, self.line));
        return tokens;
    }

    fn advance(&mut self) -> char {
        let index = self.current;
        self.current = self.current+1;
        self.source.chars().nth(index).unwrap()
    }

    fn scan_token(&mut self) -> ScanningResult {
        let char = self.advance();
        match char {
            '(' => ScanningResult::Token(TokenType::LeftParen),
            ')' => ScanningResult::Token(TokenType::RightParen),
            '{' => ScanningResult::Token(TokenType::LeftBrace),
            '}' => ScanningResult::Token(TokenType::RightBrace),
            ',' => ScanningResult::Token(TokenType::Comma),
            '.' => ScanningResult::Token(TokenType::Dot),
            '_' => ScanningResult::Token(TokenType::Minus),
            '+' => ScanningResult::Token(TokenType::Plus),
            ';' => ScanningResult::Token(TokenType::Semicolon),
            '*' => ScanningResult::Token(TokenType::Star),
            '"' => ScanningResult::Token(self.get_string_token()),
            '!' => {
                return if self.match_char('=') {
                    ScanningResult::Token(TokenType::BangEqual)
                } else {
                    ScanningResult::Token(TokenType::Bang)
                };
            },
            '=' => {
                return if self.match_char('=') {
                    ScanningResult::Token(TokenType::EqualEqual)
                } else {
                    ScanningResult::Token(TokenType::Equal)
                };
            },
            '<' => {
                return if self.match_char('=') {
                    ScanningResult::Token(TokenType::LessEqual)
                } else {
                    ScanningResult::Token(TokenType::Less)
                };
            },
            '>' => {
                return if self.match_char('=') {
                    ScanningResult::Token(TokenType::GreaterEqual)
                } else {
                    ScanningResult::Token(TokenType::Greater)
                };
            },
            '/' => return self.get_comments(),
            ' ' | '\r' | '\t'  => ScanningResult::Skip,
            '\n' => ScanningResult::NewLine,
            _ => {
                if self.is_digit(char) {
                    let number_token = self.number();
                    ScanningResult::Token(number_token)
                } else if self.is_alpha(char) {
                    let token = self.identifire();
                    ScanningResult::Token(token)
                } else {
                    ScanningResult::Error(ScannerError::UnrecognizedChar(char, self.line))
                }
            }
        }
    }


    fn add_token(&self, token_type: TokenType, tokens: &mut Vec<Token>) {
        let text = self.source[self.start..self.current].to_string();
        tokens.push(Token::new(token_type, Some(text), self.line));
    } 

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false };
        if self.source.chars().nth(self.current).unwrap() != expected { return false };
        self.current = self.current + 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn get_comments(&mut self) -> ScanningResult {
        if self.match_char('/') {
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
            return ScanningResult::Skip;
        } else {
            return ScanningResult::Token(TokenType::Slash);
        }
    }

    fn get_string_token(&mut self) -> TokenType {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            // dodać obsługe błedu "unterminated string"
            ()
        }
        self.advance();
        let string_value = self.source[self.start+1..self.current-1].to_string();
        TokenType::String(string_value)
    }

    fn identifire(&mut self) -> TokenType {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = self.source[self.start..self.current].to_string();
        let token = match TokenType::from_str(&text) {
            Ok(token) => token,
            Err(_) => TokenType::Identifire(text)
        };
        return token;
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic()
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn number(&mut self) -> TokenType{
        while self.is_digit(self.peek()) {
            self.advance();
        }
        let next_peek_char = &self.peek_next();
        if self.peek() == '.' && self.is_digit(*next_peek_char) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let number_value = self.source[self.start..self.current].to_string().parse::<f64>().unwrap();
        return TokenType::Number(number_value);

    }
}

enum ScanningResult {
    Skip,
    NewLine,
    Token(TokenType),
    Error(ScannerError)
}

#[derive(Debug)]
pub enum ScannerError {
    UnrecognizedChar(char, usize)
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScannerError::UnrecognizedChar(char, line) => write!(f, "Unrecognize char {} at line {}", char, line)
        }
    }
}
impl Error for ScannerError {}