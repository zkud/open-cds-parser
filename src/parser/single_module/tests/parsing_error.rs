use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::{ErrorCode, ParseError};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn with_wrong_syntax_it_drops_witn_a_syntax_error() {
    let source = "entity {;";

    let result = parse_single_file(&source);

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.error_code(), ErrorCode::SyntaxError);
    assert!(error.message().starts_with("Failed to parse the file"));
}

#[inline]
fn parse_single_file(file_content: &str) -> Result<Box<ModuleTerm>, ParseError> {
    let mut files = HashMap::new();
    files.insert(get_import_path(), file_content.to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    parser.parse(&get_import_path())
}

#[inline]
fn get_import_path() -> PathBuf {
    PathBuf::from("/import.cds")
}
