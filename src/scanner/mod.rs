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
            _ => ()
        }
    }

    fn add_token(&self, token_type: TokenType, tokens: &mut Vec<Token>) {
        let text = self.source[self.start..self.current].to_string();
        tokens.push(Token::new(token_type, Some(text), self.line));
    } 

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}