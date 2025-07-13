use std::fmt;

#[derive(fmt::Debug, Clone, PartialEq)]
pub enum ErrorCode {
    FileIOError,
    SyntaxError,
    LinkingError,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::FileIOError => write!(formatter, "FileIOError"),
            ErrorCode::SyntaxError => write!(formatter, "SyntaxError"),
            ErrorCode::LinkingError => write!(formatter, "LinkingError"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ErrorCode;

    #[test]
    fn with_any_code_it_formats_to_string_correctly() {
        get_all_error_codes()
            .iter()
            .zip(get_all_error_codes_names())
            .for_each(|(code, name)| assert_eq!(code.to_string(), name));
    }

    fn get_all_error_codes() -> [ErrorCode; 3] {
        [
            ErrorCode::SyntaxError,
            ErrorCode::FileIOError,
            ErrorCode::LinkingError,
        ]
    }

    fn get_all_error_codes_names() -> [&'static str; 3] {
        ["SyntaxError", "FileIOError", "LinkingError"]
    }
}
