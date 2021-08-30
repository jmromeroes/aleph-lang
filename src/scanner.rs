mod tokens;

struct Scanner {
    source: String,
    tokens: &Vec<tokens::Token>,
    start: u8,
    current: u8,
    line: u8,
    ident: u8
}

impl Scanner {
    fn new(source: String) -> Scanner {
	Scanner {
	    source: source,
	    tokens: Vec::new(),
	    start: 0,
	    current: 0,
	    line: 0,
	    ident: 0
	}
    }
    
    fn scan_tokens(&self) -> Vec<tokens::Token> {
	while !self.is_at_end() {
	    start = current;
	    self.scan_token();
	}

	self.tokens.push(Token {token_type: tokens::TokenType::EOF, lexeme: "", None, self.line});
	self.tokens
    }

    fn is_at_end(&self) -> bool {
	self.current >= source.len()
    }

    fn scan_token(&self){
	let c: char = advance();
	match c {
	    '(' => self.addToken(tokens::TokenType::LeftParen),
	    ')' => self.addToken(tokens::TokenType::RightParen),		
	    '{' => self.addToken(tokens::TokenType::LeftBrace),
	    '}' => self.addToken(tokens::TokenType::RightBrace),
	    ',' => self.addToken(tokens::TokenType::Comma),
	    '.' => self.addToken(tokens::TokenType::Dot),
	    '-' => self.addToken(tokens::TokenType::Minus),
	    '+' => self.addToken(tokens::TokenType::Plus),
	    ';' => self.addToken(tokens::TokenType::SemiColon),
	    '*' => self.addToken(tokens::TokenType::Star)
	}
    }

    fn advance() -> char {
	return self.source.chars().next().unwrap();
    }
}
