#[derive(thiserror::Error, Debug)]
pub enum ScanError {
    #[error("Error: Unexpected character '{}' at line {} col {}\n{} | {}\n", char, line_index + 1, col_index + 1, line_index + 1,line_str)]
    UnexpectedChar {
        char: String,
        line_str: String,
        line_index: usize,
        col_index: usize,
    },
    #[error("Unterminated string at line {} col {}\n{} | {}\n", line_index + 1, col_index + 1, line_index + 1,line_str)]
    UnterminatedString {
        line_str: String,
        line_index: usize,
        col_index: usize,
    },
    #[error("Unable to parse '{}' as a number at line {} col {}\n{} | {}\n", number, line_index + 1, col_index + 1, line_index + 1,line_str)]
    NumberLiteralParse {
        number: String,
        line_str: String,
        line_index: usize,
        col_index: usize,
    },
}
