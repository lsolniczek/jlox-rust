use std::str::FromStr;

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
            self.scan_token(&mut tokens);
        }

        tokens.push(Token::new(TokenType::Eof, None, self.line));
        return tokens;
    }

    fn advance(&mut self) -> char {
        let index = self.current;
        self.current = self.current+1;
        self.source.chars().nth(index).unwrap()
    }

    fn scan_token(&mut self, tokens: &mut Vec<Token>) {
        let char = self.advance();
        match char {
            '(' => self.add_token(TokenType::LeftParen, tokens),
            ')' => self.add_token(TokenType::RightParen, tokens),
            '{' => self.add_token(TokenType::LeftBrace, tokens),
            '}' => self.add_token(TokenType::RightBrace, tokens),
            ',' => self.add_token(TokenType::Comma, tokens),
            '.' => self.add_token(TokenType::Dot, tokens),
            '_' => self.add_token(TokenType::Minus, tokens),
            '+' => self.add_token(TokenType::Plus, tokens),
            ';' => self.add_token(TokenType::Semicolon, tokens),
            '*' => self.add_token(TokenType::Star, tokens),
            '"' => {
                let string_token = self.get_string_token();
                self.add_token(string_token, tokens);
            },
            '!' => {
                let token = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token, tokens);
            },
            '=' => {
                let token = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token, tokens); 
            },
            '<' => {
                let token = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token, tokens);
            },
            '>' => {
                let token = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token, tokens);
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, tokens);
                }
            },
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line = self.line + 1,
            _ => {
                if self.is_digit(char) {
                    let number_token = self.number();
                    self.add_token(number_token, tokens);
                } else if self.is_alpha(char) {
                    let token = self.identifire();
                    self.add_token(token, tokens);
                } else {
                    () // dopisać tu obsługe błędów
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
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let number_value = self.source[self.start..self.current].to_string().parse::<f64>().unwrap();
        return TokenType::Number(number_value);

    }
}