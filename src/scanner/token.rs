use ahash::AHashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KEYWORDS: AHashMap<&'static str, Token> = {
        let mut k = AHashMap::new();
        k.insert("and", Token::And);
        k.insert("class", Token::Class);
        k.insert("else", Token::Else);
        k.insert("false", Token::False);
        k.insert("for", Token::For);
        k.insert("fun", Token::Fun);
        k.insert("if", Token::If);
        k.insert("nil", Token::Nil);
        k.insert("or", Token::Or);
        k.insert("print", Token::Print);
        k.insert("return", Token::Return);
        k.insert("super", Token::Super);
        k.insert("this", Token::This);
        k.insert("true", Token::True);
        k.insert("var", Token::Var);
        k.insert("while", Token::While);
        k
    };
}

#[derive(Debug, Clone)]
pub enum Token {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier(String),
    String(String),
    Number(f64),
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}
