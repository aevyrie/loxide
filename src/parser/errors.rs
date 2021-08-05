#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Error: failed to pop token off the stack")]
    PopFailed,
    #[error("Error: Missing right parenthesis")]
    MissingRightParen,
    #[error("Error: unexpected token found")]
    UnexpectedToken,
}
