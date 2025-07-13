use open_cds_parser::ast::*;
use open_cds_parser::parser::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[inline]
fn setup_parser() -> Parser {
    Parser::new_with_native_fs()
}

#[inline]
fn parse_module(path: &str) -> Result<Box<ModuleTerm>, ParseError> {
    setup_parser().parse_single_module(&PathBuf::from(path))
}

#[inline]
fn parse_multiple_modules(path: &str) -> Result<HashMap<PathBuf, ModuleTerm>, ParseError> {
    setup_parser().parse_multiple_modules(vec![PathBuf::from(path)])
}

#[test]
fn with_syntax_errors_it_returns_a_readable_error() {
    let result = parse_module("./tests/projects/errors/srv/syntaxerror.cds");

    let expected_error_message="Error(SyntaxError): Failed to parse the file \"./tests/projects/errors/srv/syntaxerror.cds\".
Please see the internal parser error:";
    assert!(result
        .unwrap_err()
        .to_string()
        .starts_with(expected_error_message));
}

#[test]
fn with_linking_errors_it_returns_a_readable_error() {
    let result = parse_multiple_modules("./tests/projects/errors/srv/linkingerror.cds");

    let expected_error_message="Error(LinkingError): Cannot find the import './tests/projects/errors/srv/./missingpath' from ./tests/projects/errors/srv/linkingerror.cds";
    assert_eq!(result.unwrap_err().to_string(), expected_error_message);
}
