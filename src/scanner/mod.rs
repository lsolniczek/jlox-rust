use tokens::{TokenType, Token};

pub mod tokens;

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { 
            source: source,
            tokens: vec![],
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

        tokens.push(Token::new(TokenType::Eof, "".to_string(), self.line));
        return tokens;
    }

    fn scan_token(&self, tokens: &mut Vec<Token>) {

    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}