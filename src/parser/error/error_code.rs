use std::fmt;

#[derive(fmt::Debug, Clone, PartialEq)]
pub enum ErrorCode {
    FileIOError,
    SyntaxError,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::FileIOError => write!(formatter, "FileIOError"),
            ErrorCode::SyntaxError => write!(formatter, "SyntaxError"),
        }
    }
}
