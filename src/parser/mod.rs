use self::{errors::ParseError, expressions::Expression};
use crate::scanner::tokens::Token::{self, *};

pub mod errors;
pub mod expressions;

pub fn parse(tokens: &mut Vec<Token>) {
    tokens.reverse(); // Reverse so we can pop() off the "front" of the vec
    let mut errors = Vec::new();
    let mut expression_list = Vec::new();
    expression_list.push(expression(tokens, &mut errors).map_err(|expr| {
        synchronize(tokens);
        expr
    }));
    for error in errors {
        println!("{}", error);
    }
    for expr in expression_list {
        match expr {
            Ok(x) => println!("Parse Succeeded: {}", x),
            Err(Some(x)) => println!("Parse Error: {}", x),
            _ => (),
        }
    }
}

fn synchronize(tokens: &mut Vec<Token>) {
    while let Some(token) = tokens.iter().peekable().peek() {
        match token {
            Semicolon => {
                tokens.pop();
                return;
            }
            Class | Fun | Var | For | If | While | Print | Return => return,
            _ => (),
        }
        tokens.pop();
    }
}

fn pop_val(tokens: &mut Vec<Token>, errors: &mut Vec<ParseError>) -> Result<Token, ()> {
    match tokens.pop() {
        Some(t) => Ok(t),
        None => {
            errors.push(ParseError::PopFailed);
            return Err(());
        }
    }
}

fn expression(
    tokens: &mut Vec<Token>,
    errors: &mut Vec<ParseError>,
) -> Result<Expression, Option<Expression>> {
    equality(tokens, errors)
}

fn equality(
    tokens: &mut Vec<Token>,
    errors: &mut Vec<ParseError>,
) -> Result<Expression, Option<Expression>> {
    let mut expr = comparison(tokens, errors)?;
    while let Some(BangEqual | Equal) = tokens.last() {
        let operator = pop_val(tokens, errors).map_err(|_| expr.clone())?;
        let right = comparison(tokens, errors)?;
        expr = Expression::binary(expr, operator, right);
    }
    Ok(expr)
}

fn comparison(
    tokens: &mut Vec<Token>,
    errors: &mut Vec<ParseError>,
) -> Result<Expression, Option<Expression>> {
    let mut expr = term(tokens, errors)?;
    while let Some(Greater | GreaterEqual | Less | LessEqual) = tokens.last() {
        let operator = pop_val(tokens, errors).map_err(|_| expr.clone())?;
        let right = term(tokens, errors)?;
        expr = Expression::binary(expr, operator, right);
    }
    Ok(expr)
}

fn term(
    tokens: &mut Vec<Token>,
    errors: &mut Vec<ParseError>,
) -> Result<Expression, Option<Expression>> {
    let mut expr = factor(tokens, errors)?;
    while let Some(Minus | Plus) = tokens.last() {
        let operator = pop_val(tokens, errors).map_err(|_| expr.clone())?;
        let right = factor(tokens, errors)?;
        expr = Expression::binary(expr, operator, right);
    }
    Ok(expr)
}

fn factor(
    tokens: &mut Vec<Token>,
    errors: &mut Vec<ParseError>,
) -> Result<Expression, Option<Expression>> {
    let mut expr = unary(tokens, errors)?;
    while let Some(Slash | Star) = tokens.last() {
        let operator = pop_val(tokens, errors).map_err(|_| expr.clone())?;
        let right = unary(tokens, errors)?;
        expr = Expression::binary(expr, operator, right);
    }
    Ok(expr)
}

fn unary(
    tokens: &mut Vec<Token>,
    errors: &mut Vec<ParseError>,
) -> Result<Expression, Option<Expression>> {
    if let Some(Bang | Minus) = tokens.last() {
        let operator = pop_val(tokens, errors).map_err(|_| None)?;
        let right = unary(tokens, errors)?;
        return Ok(Expression::unary(operator, right));
    }
    primary(tokens, errors)
}

fn primary(
    tokens: &mut Vec<Token>,
    errors: &mut Vec<ParseError>,
) -> Result<Expression, Option<Expression>> {
    let token = tokens.pop().unwrap();
    if let False = token {
        return Ok(Expression::literal_bool(false));
    }
    if let True = token {
        return Ok(Expression::literal_bool(true));
    }
    if let Number(n) = token {
        return Ok(Expression::literal_num(n));
    }
    if let String(s) = token {
        return Ok(Expression::literal_str(s));
    }
    if let LeftParen = token {
        let expr = expression(tokens, errors)?;
        if let Some(RightParen) = tokens.pop() {
            return Ok(Expression::grouping(expr));
        } else {
            errors.push(ParseError::MissingRightParen);
            return Err(Some(expr));
        }
    }
    errors.push(ParseError::ExpectedExpression);
    Err(None)
}
