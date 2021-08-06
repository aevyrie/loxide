use crate::scanner::tokens::Token;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Expression {
    LiteralStr(Literal<String>),
    LiteralNum(Literal<f64>),
    LiteralBool(Literal<bool>),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
}
impl Expression {
    pub fn literal_str(value: String) -> Expression {
        Expression::LiteralStr(Literal { value })
    }
    pub fn literal_num(value: f64) -> Expression {
        Expression::LiteralNum(Literal { value })
    }
    pub fn literal_bool(value: bool) -> Expression {
        Expression::LiteralBool(Literal { value })
    }
    pub fn unary(operator: Token, right: Expression) -> Expression {
        Expression::Unary(Unary {
            operator,
            right: Box::new(right),
        })
    }
    pub fn binary(left: Expression, operator: Token, right: Expression) -> Expression {
        Expression::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    pub fn grouping(expression: Expression) -> Expression {
        Expression::Grouping(Grouping {
            expression: Box::new(expression),
        })
    }
}
impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::LiteralStr(s) => Display::fmt(&s, f),
            &Self::LiteralNum(s) => Display::fmt(&s, f),
            &Self::LiteralBool(s) => Display::fmt(&s, f),
            &Self::Unary(s) => Display::fmt(&s, f),
            &Self::Binary(s) => Display::fmt(&s, f),
            &Self::Grouping(s) => Display::fmt(&s, f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}
impl Display for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expression>,
}
impl Display for Grouping {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.expression)
    }
}

#[derive(Debug, Clone)]
pub struct Literal<T: Display + Debug> {
    pub value: T,
}
impl<T: Display + Debug> Display for Literal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expression>,
}
impl Display for Unary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}
