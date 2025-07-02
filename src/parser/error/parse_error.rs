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
    fn with_any_possible_error_it_provides_correct_messages() {
        get_all_possible_parse_errors()
            .iter()
            .zip(get_expected_messages())
            .for_each(|(error, message)| assert_eq!(error.get_message(), message));
    }

    fn get_all_possible_parse_errors() -> Vec<ParseError> {
        vec![
            ParseError::new(ErrorCode::FileIOError, "file error".to_string()),
            ParseError::new(ErrorCode::SyntaxError, "syntax error".to_string()),
            ParseError::new(ErrorCode::LinkingError, "linking error".to_string()),
        ]
    }

    fn get_expected_messages() -> [&'static str; 3] {
        ["file error", "syntax error", "linking error"]
    }

    #[test]
    fn with_any_possible_error_it_provides_correct_codes() {
        get_all_possible_parse_errors()
            .iter()
            .zip(get_all_possible_parse_error_codes())
            .for_each(|(error, code)| assert_eq!(error.get_error_type(), code));
    }

    fn get_all_possible_parse_error_codes() -> [ErrorCode; 3] {
        [
            ErrorCode::FileIOError,
            ErrorCode::SyntaxError,
            ErrorCode::LinkingError,
        ]
    }

    #[test]
    fn with_all_possible_error_messages_it_converts_to_string_correctly() {
        get_all_possible_parse_errors()
            .iter()
            .zip(get_all_string_convertions())
            .for_each(|(error, expected_string_convertion)| {
                assert_eq!(error.to_string(), expected_string_convertion)
            });
    }

    fn get_all_string_convertions() -> [&'static str; 3] {
        [
            "Error(FileIOError): file error",
            "Error(SyntaxError): syntax error",
            "Error(LinkingError): linking error",
        ]
    }
}
