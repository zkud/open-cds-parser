use super::super::*;
use super::utils::*;
use crate::ast::*;
use crate::parser::error::ParseError;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::SingleModuleParser;
use crate::parser::ErrorCode;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

struct MockSingleModuleParser;

impl SingleModuleParser for MockSingleModuleParser {
    fn parse(&self, path: &Path) -> Result<Box<ModuleTerm>, ParseError> {
        if path == get_failure_no_file_present_path() {
            return Ok(Box::new(ModuleTerm::new(
                get_mock_location(),
                vec![
                    ModuleDefinition::Import(ImportTerm::new(
                        get_mock_location(),
                        Box::new(KeywordTerm::new(get_mock_location(), Keyword::Using)),
                        Box::new(SelectionBlockTerm::new(
                            get_mock_location(),
                            None,
                            vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                                get_mock_location(),
                                Box::new(ImportIdentifierTerm::new(
                                    get_mock_location(),
                                    Box::new(ImportIdentifierVariant::IdentifierOnly(Box::new(
                                        IdentifierTerm::new_basic(get_mock_location(), "Books"),
                                    ))),
                                )),
                                None,
                                None,
                            ))],
                            None,
                        )),
                        Box::new(KeywordTerm::new(get_mock_location(), Keyword::From)),
                        Box::new(PathTerm::new(
                            get_mock_location(),
                            PathBuf::from("/no_file_present"),
                        )),
                        Box::new(PunctuationSignTerm::new(
                            get_mock_location(),
                            PunctuationSign::Semicolumn,
                        )),
                    )),
                    ModuleDefinition::Service(ServiceDeclarationTerm::new(
                        get_mock_location(),
                        Box::new(KeywordTerm::new(get_mock_location(), Keyword::Service)),
                        Box::new(IdentifierTerm::new_basic(
                            get_mock_location(),
                            "BooksService",
                        )),
                        Box::new(PunctuationSignTerm::new(
                            get_mock_location(),
                            PunctuationSign::OpenCurlyBrace,
                        )),
                        vec![],
                        Box::new(PunctuationSignTerm::new(
                            get_mock_location(),
                            PunctuationSign::CloseCurlyBrace,
                        )),
                    )),
                ],
            )));
        }
        return Err(ParseError::new(
            ErrorCode::FileIOError,
            "Unexpected file".to_string(),
        ));
    }
}

#[inline]
pub fn get_failure_no_file_present_path() -> PathBuf {
    PathBuf::from("/failure_no_file_present.cds")
}

#[test]
fn test_parse_invalid_path() {
    let directories = HashMap::new();
    let files = HashMap::new();

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec![PathBuf::from("invalid")]);

    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_path_in_import() {
    let mut directories = HashMap::new();
    directories.insert(
        PathBuf::from("/"),
        vec![PathBuf::from("/failure_no_file_present.cds")],
    );

    let mut files = HashMap::new();
    files.insert(
        PathBuf::from("/failure_no_file_present.cds"),
        "".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec![get_failure_no_file_present_path()]);

    assert!(result.is_err());
    let parse_error = result.err().unwrap();
    assert_eq!(parse_error.error_code(), ErrorCode::LinkingError);
}
