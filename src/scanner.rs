use crate::token::{self, Token};
use unicode_segmentation::UnicodeSegmentation;

#[derive(thiserror::Error, Debug)]
pub enum ScanError {
    #[error("Error: Unexpected character '{}' at line {} col {}\n{} | {}\n", char, line_index + 1, col_index + 1, line_index + 1,line)]
    UnexpectedChar {
        char: String,
        line: String,
        line_index: usize,
        col_index: usize,
    },
    #[error("Unterminated string at line {} col {}\n{} | {}\n", line_index + 1, col_index + 1, line_index + 1,line)]
    UnterminatedString {
        line: String,
        line_index: usize,
        col_index: usize,
    },
}

pub struct Scanner {
    //source: String,
    tokens: Result<Vec<Token>, Vec<ScanError>>,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        let tokens = Scanner::scan_tokens(&source);
        Self { tokens }
    }

    pub fn scan_tokens(source: &str) -> Result<Vec<Token>, Vec<ScanError>> {
        let mut scan_errors = Vec::new();
        let mut tokens = Vec::new();
        let mut lines = source.lines().enumerate();
        while let Some((mut line, mut line_string)) = lines.next() {
            let mut graphemes = line_string.graphemes(true).enumerate().peekable();
            while let Some((start, char)) = graphemes.next() {
                match char {
                    "(" => tokens.push(Token::LeftParen),
                    ")" => tokens.push(Token::RightParen),
                    "{" => tokens.push(Token::LeftBrace),
                    "}" => tokens.push(Token::RightBrace),
                    "," => tokens.push(Token::Comma),
                    "." => tokens.push(Token::Dot),
                    "-" => tokens.push(Token::Minus),
                    "+" => tokens.push(Token::Plus),
                    ";" => tokens.push(Token::Semicolon),
                    "*" => tokens.push(Token::Star),
                    "!" => {
                        if let Some((_, "=")) = graphemes.peek() {
                            tokens.push(Token::BangEqual);
                            graphemes.next();
                        } else {
                            tokens.push(Token::Bang);
                        }
                    }
                    "=" => {
                        if let Some((_, "=")) = graphemes.peek() {
                            tokens.push(Token::EqualEqual);
                            graphemes.next();
                        } else {
                            tokens.push(Token::Equal);
                        }
                    }
                    "<" => {
                        if let Some((_, "=")) = graphemes.peek() {
                            tokens.push(Token::LessEqual);
                            graphemes.next();
                        } else {
                            tokens.push(Token::Less);
                        }
                    }
                    ">" => {
                        if let Some((_, "=")) = graphemes.peek() {
                            tokens.push(Token::GreaterEqual);
                            graphemes.next();
                        } else {
                            tokens.push(Token::Greater);
                        }
                    }
                    "/" => {
                        if let Some((_, "/")) = graphemes.peek() {
                            graphemes.next();
                            while graphemes.next_if(|&c| c.1 != "\n").is_some() {}
                        } else {
                            tokens.push(Token::Slash);
                        }
                    }
                    "\"" => {
                        let mut string_literal = String::from("");
                        loop {
                            if let Some((_, char)) = graphemes.next() {
                                if char == "\"" {
                                    break;
                                } else {
                                    string_literal = [string_literal, char.into()].concat();
                                }
                            } else if let Some((next_line, next_line_string)) = lines.next() {
                                // This section makes multi-line strings possible
                                line = next_line;
                                line_string = next_line_string;
                                graphemes = next_line_string.graphemes(true).enumerate().peekable();
                            } else {
                                scan_errors.push(ScanError::UnterminatedString {
                                    line: line_string.into(),
                                    line_index: line,
                                    col_index: start,
                                });
                                break;
                            }
                        }
                        tokens.push(Token::String(string_literal));
                    }
                    " " => (), // Ignore whitespace
                    c => {
                        if Scanner::is_digit(c) {
                            let mut number_literal = String::from(c);
                            'outer: while let Some((_, next_char)) = graphemes.peek() {
                                if Scanner::is_digit(next_char) {
                                    number_literal = [number_literal, (*next_char).into()].concat();
                                    graphemes.next(); // Collect integer part
                                } else if *next_char == "." {
                                    number_literal = [number_literal, (*next_char).into()].concat();
                                    graphemes.next(); // Collect the decimal
                                    while let Some((_, next_next_char)) = graphemes.peek() {
                                        if Scanner::is_digit(next_next_char) {
                                            number_literal =
                                                [number_literal, (*next_next_char).into()].concat();
                                            graphemes.next(); // Collect the fractional part
                                        } else {
                                            break 'outer;
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                            let number: f64 = number_literal.parse().unwrap();
                            tokens.push(Token::Number(number));
                        } else if Scanner::is_alpha(c) {
                            let mut identifier = c;
                            #[allow(unused_assignments)]
                            let mut id_str = identifier.into();
                            while let Some((_, next_char)) = graphemes.peek() {
                                if Scanner::is_alphanumeric(next_char) {
                                    id_str = [identifier, *next_char].concat();
                                    identifier = &id_str;
                                    graphemes.next();
                                } else {
                                    break;
                                }
                            }
                            if let Some(keyword) = token::KEYWORDS.get(identifier) {
                                tokens.push(keyword.clone());
                            } else {
                                tokens.push(Token::Identifier(String::from(identifier)));
                            }
                        } else {
                            scan_errors.push(ScanError::UnexpectedChar {
                                char: c.into(),
                                line: line_string.into(),
                                line_index: line,
                                col_index: start,
                            });
                        }
                    }
                }
            }
        }
        tokens.push(Token::Eof);
        if scan_errors.is_empty() {
            Ok(tokens)
        } else {
            Err(scan_errors)
        }
    }

    fn is_digit(char: &str) -> bool {
        matches!(
            char,
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
        )
    }

    fn is_alpha(char: &str) -> bool {
        if let Some(c) = char.chars().next() {
            c.is_alphabetic()
        } else {
            false
        }
    }

    fn is_alphanumeric(char: &str) -> bool {
        if let Some(c) = char.chars().next() {
            c.is_alphanumeric()
        } else {
            false
        }
    }

    /// Get a reference to the scanner's tokens.
    pub fn tokens(&self) -> &Result<Vec<Token>, Vec<ScanError>> {
        &self.tokens
    }
}
