use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicollon, Star,

    // One or two character tokens.
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Arrow, FunArrow, SemiColon,

    // Literals
    Identifier, String, Integer, Double, Float, Long,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Const, Var, While,
    
    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "{:?}", self)
    }
}

pub struct Token <'a>{
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: u8
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	f.write_str(&format!("{} {}", self.token_type, self.lexeme))
    }
}
