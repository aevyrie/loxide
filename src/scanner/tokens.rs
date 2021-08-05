use std::fmt::{Display, Formatter};

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

// TODO: Would be good to make each of these more type constrained? E.g. maybe this enum contains
// boxes that reference these variants as actual types which implement traits to separate, say,
// operators from other tokens.

#[derive(Debug, Clone, PartialEq)]
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

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Semicolon => write!(f, ";"),
            Self::Slash => write!(f, "/"),
            Self::Star => write!(f, "*"),
            // One or two character
            Self::Bang => write!(f, "!"),
            Self::BangEqual => write!(f, "!="),
            Self::Equal => write!(f, "="),
            Self::EqualEqual => write!(f, "=="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Less => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            // Literals
            Self::Identifier(s) | Self::String(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{}", n),
            // Keywords
            Self::And => write!(f, "And"),
            Self::Class => write!(f, "Class"),
            Self::Else => write!(f, "Else"),
            Self::False => write!(f, "False"),
            Self::Fun => write!(f, "Fun"),
            Self::For => write!(f, "For"),
            Self::If => write!(f, "If"),
            Self::Nil => write!(f, "Nil"),
            Self::Or => write!(f, "Or"),
            Self::Print => write!(f, "Print"),
            Self::Return => write!(f, "Return"),
            Self::Super => write!(f, "Super"),
            Self::This => write!(f, "This"),
            Self::True => write!(f, "True"),
            Self::Var => write!(f, "Var"),
            Self::While => write!(f, "While"),
            Self::Eof => write!(f, "Eof"),
        }
    }
}
