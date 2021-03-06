pub mod errors;
pub mod tokens;

use self::{
    errors::ScanError,
    tokens::{Token, KEYWORDS},
};
use unicode_segmentation::UnicodeSegmentation;

pub fn scan(source: &str) -> Result<Vec<Token>, Vec<ScanError>> {
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
                                line_str: line_string.into(),
                                line_index: line,
                                col_index: start,
                            });
                            break;
                        }
                    }
                    tokens.push(Token::String(string_literal));
                }
                " " => (), // Ignore whitespace
                other_char => {
                    if is_digit(other_char) {
                        let mut number_literal = String::from(other_char);
                        'outer: while let Some((_, next_char)) = graphemes.peek() {
                            if is_digit(next_char) {
                                number_literal = [number_literal, (*next_char).into()].concat();
                                graphemes.next(); // Collect integer part
                            } else if *next_char == "." {
                                number_literal = [number_literal, (*next_char).into()].concat();
                                graphemes.next(); // Collect the decimal
                                while let Some((_, next_next_char)) = graphemes.peek() {
                                    if is_digit(next_next_char) {
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
                        match number_literal.parse() {
                            Ok(num) => {
                                let number: f64 = num;
                                tokens.push(Token::Number(number));
                            }
                            Err(_) => scan_errors.push(ScanError::NumberLiteralParse {
                                number: number_literal,
                                line_str: line_string.into(),
                                line_index: line,
                                col_index: start,
                            }),
                        }
                    } else if is_alpha(other_char) {
                        let mut identifier = other_char;
                        #[allow(unused_assignments)]
                        let mut id_str = identifier.into();
                        while let Some((_, next_char)) = graphemes.peek() {
                            if is_alphanumeric(next_char) {
                                id_str = [identifier, *next_char].concat();
                                identifier = &id_str;
                                graphemes.next();
                            } else {
                                break;
                            }
                        }
                        if let Some(keyword) = KEYWORDS.get(identifier) {
                            tokens.push(keyword.clone());
                        } else {
                            tokens.push(Token::Identifier(String::from(identifier)));
                        }
                    } else {
                        scan_errors.push(ScanError::UnexpectedChar {
                            char: other_char.into(),
                            line_str: line_string.into(),
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
    if let Some(c) = char.chars().next() {
        c.is_numeric()
    } else {
        false
    }
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
