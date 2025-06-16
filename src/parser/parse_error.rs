use std::fmt;

use super::fs::FileSystemError;

#[derive(fmt::Debug)]
pub struct ParseError {
    error_type: ParseErrorType,
    message: String,
}

#[derive(fmt::Debug, Clone, PartialEq)]
pub enum ParseErrorType {
    FileIOError,
    SyntaxError,
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorType::FileIOError => write!(formatter, "FileIOError"),
            ParseErrorType::SyntaxError => write!(formatter, "SyntaxError"),
        }
    }
}

impl ParseError {
    pub fn new(message: String, error_type: ParseErrorType) -> ParseError {
        ParseError {
            message,
            error_type,
        }
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_error_type(&self) -> ParseErrorType {
        self.error_type.clone()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Error({}): {}", self.error_type, self.message)
    }
}

impl From<FileSystemError> for ParseError {
    fn from(error: FileSystemError) -> ParseError {
        ParseError::new(format!("{}", error), ParseErrorType::FileIOError)
    }
}

#[cfg(test)]
mod tests {
    use super::ParseError;
    use super::ParseErrorType;

    #[test]
    fn it_inits() {
        let file_error = ParseError::new("file error".to_string(), ParseErrorType::FileIOError);
        let syntax_error = ParseError::new("syntax error".to_string(), ParseErrorType::SyntaxError);

        assert_eq!(file_error.get_message(), "file error");
        assert_eq!(file_error.get_error_type(), ParseErrorType::FileIOError);

        assert_eq!(syntax_error.get_message(), "syntax error");
        assert_eq!(syntax_error.get_error_type(), ParseErrorType::SyntaxError);
    }

    #[test]
    fn it_displayable() {
        let file_error = ParseError::new("file error".to_string(), ParseErrorType::FileIOError);
        let syntax_error = ParseError::new("syntax error".to_string(), ParseErrorType::SyntaxError);

        assert_eq!(format!("{}", file_error), "Error(FileIOError): file error");
        assert_eq!(
            format!("{}", syntax_error),
            "Error(SyntaxError): syntax error"
        );
    }
}
