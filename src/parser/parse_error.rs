use std::fmt;

use super::fs::FileSystemError;

#[derive(fmt::Debug)]
pub struct ParseError {
    error_code: ErrorCode,
    message: String,
}

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

impl ParseError {
    pub fn new(message: String, error_code: ErrorCode) -> ParseError {
        ParseError {
            message,
            error_code,
        }
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_error_type(&self) -> ErrorCode {
        self.error_code.clone()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Error({}): {}", self.error_code, self.message)
    }
}

impl From<FileSystemError> for ParseError {
    fn from(error: FileSystemError) -> ParseError {
        ParseError::new(format!("{}", error), ErrorCode::FileIOError)
    }
}

#[cfg(test)]
mod tests {
    use super::ErrorCode;
    use super::ParseError;

    #[test]
    fn it_inits() {
        let file_error = ParseError::new("file error".to_string(), ErrorCode::FileIOError);
        let syntax_error = ParseError::new("syntax error".to_string(), ErrorCode::SyntaxError);

        assert_eq!(file_error.get_message(), "file error");
        assert_eq!(file_error.get_error_type(), ErrorCode::FileIOError);

        assert_eq!(syntax_error.get_message(), "syntax error");
        assert_eq!(syntax_error.get_error_type(), ErrorCode::SyntaxError);
    }

    #[test]
    fn it_displayable() {
        let file_error = ParseError::new("file error".to_string(), ErrorCode::FileIOError);
        let syntax_error = ParseError::new("syntax error".to_string(), ErrorCode::SyntaxError);

        assert_eq!(format!("{}", file_error), "Error(FileIOError): file error");
        assert_eq!(
            format!("{}", syntax_error),
            "Error(SyntaxError): syntax error"
        );
    }
}
