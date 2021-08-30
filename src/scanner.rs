use crate::{
    tokens::{Token, TokenType}
};

pub struct Scanner <'a>{
    source: String,
    tokens: Vec<Token<'a>>,
    start: u8,
    current: u8,
    line: u8,
    ident: u8
}

impl Scanner<'_>{
    pub fn new(source: String) -> Self {
	Scanner {
	    source: source,
	    tokens: Vec::<Token>::new(),
	    start: 0,
	    current: 0,
	    line: 0,
	    ident: 0
	}
    }
    
    fn scan_tokens(&self) -> Vec<Token> {
	while !self.is_at_end() {
	    self.start = self.current;
	    self.scan_token();
	}

	self.tokens.push(Token {token_type: TokenType::EOF, lexeme: "", line: self.line});
	self.tokens
    }

    fn is_at_end(&self) -> bool {
	self.current >= self.source.len() as u8
    }

    fn scan_token(&self){
	let c: char = self.advance();
	match c {
	    '(' => self.add_token(TokenType::LeftParen),
	    ')' => self.add_token(TokenType::RightParen),		
	    '{' => self.add_token(TokenType::LeftBrace),
	    '}' => self.add_token(TokenType::RightBrace),
	    ',' => self.add_token(TokenType::Comma),
	    '.' => self.add_token(TokenType::Dot),
	    '-' => self.add_token(TokenType::Minus),
	    '+' => self.add_token(TokenType::Plus),
	    ';' => self.add_token(TokenType::SemiColon),
	    '*' => self.add_token(TokenType::Star)
	}
    }

    fn advance(&self) -> char {
	self.source.chars().next().unwrap()
    }

    fn add_token(&self, token: TokenType){
	
    }
}
