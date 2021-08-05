use self::{errors::ParseError, expressions::Expression};
use crate::scanner::tokens::Token::{self, *};
use std::error::Error;

pub mod errors;
pub mod expressions;

pub fn parse(tokens: &mut Vec<Token>) -> Result<Expression, Box<dyn Error>> {
    tokens.reverse(); // Reverse so we can pop() off the "front" of the vec
    expression(tokens)
}

fn expression(tokens: &mut Vec<Token>) -> Result<Expression, Box<dyn Error>> {
    equality(tokens)
}

fn equality(tokens: &mut Vec<Token>) -> Result<Expression, Box<dyn Error>> {
    let mut expr = comparison(tokens)?;
    while let Some(BangEqual | Equal) = tokens.last() {
        let operator = tokens.pop().ok_or(ParseError::PopFailed)?;
        let right = comparison(tokens)?;
        expr = Expression::binary(expr, operator, right);
    }
    Ok(expr)
}

fn comparison(tokens: &mut Vec<Token>) -> Result<Expression, Box<dyn Error>> {
    let mut expr = term(tokens)?;
    while let Some(Greater | GreaterEqual | Less | LessEqual) = tokens.last() {
        let operator = tokens.pop().ok_or(ParseError::PopFailed)?;
        let right = term(tokens)?;
        expr = Expression::binary(expr, operator, right);
    }
    Ok(expr)
}

fn term(tokens: &mut Vec<Token>) -> Result<Expression, Box<dyn Error>> {
    let mut expr = factor(tokens)?;
    while let Some(Minus | Plus) = tokens.last() {
        let operator = tokens.pop().ok_or(ParseError::PopFailed)?;
        let right = factor(tokens)?;
        expr = Expression::binary(expr, operator, right);
    }
    Ok(expr)
}

fn factor(tokens: &mut Vec<Token>) -> Result<Expression, Box<dyn Error>> {
    let mut expr = unary(tokens)?;
    while let Some(Slash | Star) = tokens.last() {
        let operator = tokens.pop().ok_or(ParseError::PopFailed)?;
        let right = unary(tokens)?;
        expr = Expression::binary(expr, operator, right);
    }
    Ok(expr)
}

fn unary(tokens: &mut Vec<Token>) -> Result<Expression, Box<dyn Error>> {
    if let Some(Bang | Minus) = tokens.last() {
        let operator = tokens.pop().ok_or(ParseError::PopFailed)?;
        let right = unary(tokens)?;
        return Ok(Expression::unary(operator, right));
    }
    primary(tokens)
}

fn primary(tokens: &mut Vec<Token>) -> Result<Expression, Box<dyn Error>> {
    let token = tokens.pop();
    if let Some(False) = token {
        return Ok(Expression::literal_bool(false));
    }
    if let Some(True) = token {
        return Ok(Expression::literal_bool(true));
    }
    if let Some(Number(n)) = token {
        return Ok(Expression::literal_num(n));
    }
    if let Some(String(s)) = token {
        return Ok(Expression::literal_str(s));
    }
    if let Some(LeftParen) = token {
        let expr = expression(tokens)?;
        if let Some(RightParen) = tokens.iter().next() {
            return Ok(Expression::grouping(expr));
        } else {
            return Err(ParseError::MissingRightParen.into());
        }
    }
    Err(ParseError::UnexpectedToken.into())
}
