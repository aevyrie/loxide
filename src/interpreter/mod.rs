use crate::{parser::expressions::Expression, scanner::tokens::Token};
use core::panic;

pub trait Interpretation<T> {
    fn interpret(&self) -> T;
}

impl Interpretation<bool> for Expression {
    fn interpret(&self) -> bool {
        match self {
            Expression::Unary(u) => {
                if let Token::Bang = u.operator {
                    let result: bool = u.right.interpret();
                    !result
                } else {
                    panic!(
                        "Expected '!' on LHS of unary expression, found {}",
                        u.operator
                    )
                }
            }
            _ => panic!(),
        }
    }
}

impl Interpretation<f64> for Expression {
    fn interpret(&self) -> f64 {
        match self {
            Expression::Unary(u) => {
                if let Token::Minus = u.operator {
                    let result: f64 = u.right.interpret();
                    -result
                } else {
                    panic!(
                        "Expected '-' on LHS of unary expression, found {}",
                        u.operator
                    )
                }
            }
            Expression::Binary(b) => match b.operator {
                Token::Minus => {
                    let lhs: f64 = b.left.interpret();
                    let rhs: f64 = b.right.interpret();
                    lhs - rhs
                }
                Token::Slash => {
                    let lhs: f64 = b.left.interpret();
                    let rhs: f64 = b.right.interpret();
                    lhs / rhs
                }
                Token::Star => {
                    let lhs: f64 = b.left.interpret();
                    let rhs: f64 = b.right.interpret();
                    lhs * rhs
                }
                Token::Plus => {
                    let lhs: f64 = b.left.interpret();
                    let rhs: f64 = b.right.interpret();
                    lhs + rhs
                }
                _ => panic!("Unexpected operator in binary statement"),
            },
            Expression::LiteralNum(n) => n.value,
            _ => panic!(&format!("{:?}", self)),
        }
    }
}

impl Interpretation<String> for Expression {
    fn interpret(&self) -> String {
        match self {
            Expression::Binary(b) => match b.operator {
                Token::Plus => {
                    let lhs: String = b.left.interpret();
                    let rhs: String = b.right.interpret();
                    format!("{}{}", lhs, rhs)
                }
                _ => panic!("Unexpected operator in binary statement"),
            },
            Expression::LiteralStr(s) => String::from(s.value.clone()),
            _ => panic!(&format!("{:?}", self)),
        }
    }
}
