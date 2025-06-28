use std::fmt;

use super::super::fs::FileSystemError;
use super::ErrorCode;

#[derive(fmt::Debug)]
pub struct ParseError {
    error_code: ErrorCode,
    message: String,
}

impl ParseError {
    pub fn new(error_code: ErrorCode, message: String) -> ParseError {
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
        ParseError::new(ErrorCode::FileIOError, format!("{}", error))
    }
}

#[cfg(test)]
mod tests {
    use super::ErrorCode;
    use super::ParseError;

    #[test]
    fn it_inits() {
        let file_error = ParseError::new(ErrorCode::FileIOError, "file error".to_string());
        let syntax_error = ParseError::new(ErrorCode::SyntaxError, "syntax error".to_string());

        assert_eq!(file_error.get_message(), "file error");
        assert_eq!(file_error.get_error_type(), ErrorCode::FileIOError);

        assert_eq!(syntax_error.get_message(), "syntax error");
        assert_eq!(syntax_error.get_error_type(), ErrorCode::SyntaxError);
    }

    #[test]
    fn it_displayable() {
        let file_error = ParseError::new(ErrorCode::FileIOError, "file error".to_string());
        let syntax_error = ParseError::new(ErrorCode::SyntaxError, "syntax error".to_string());

        assert_eq!(format!("{}", file_error), "Error(FileIOError): file error");
        assert_eq!(
            format!("{}", syntax_error),
            "Error(SyntaxError): syntax error"
        );
    }
}
