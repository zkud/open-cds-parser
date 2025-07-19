use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[inline]
pub fn parse_single_file(file_content: &str) -> Result<Box<ModuleTerm>, ParseError> {
    let mut files = HashMap::new();
    files.insert(get_mock_path(), file_content.to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    parser.parse(&get_mock_path())
}

#[inline]
pub fn get_mock_path() -> PathBuf {
    PathBuf::from("/mock.cds")
}
