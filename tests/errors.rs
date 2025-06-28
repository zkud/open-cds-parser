use open_cds_parser::ast::*;
use open_cds_parser::parser::*;
use std::path::PathBuf;

#[inline]
fn setup_parser() -> Parser {
    Parser::new_with_native_fs()
}

#[inline]
fn parse_module(path: &str) -> Result<Box<ModuleTerm>, ParseError> {
    setup_parser().parse_single_module(&PathBuf::from(path))
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
