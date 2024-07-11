use tokens::Tokens;

pub mod tokens;

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn scan_tokens(&self) -> Vec<Tokens> {
        vec![]
    }
}