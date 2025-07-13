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

struct MockSingleModuleParserForDuplication;

impl SingleModuleParser for MockSingleModuleParserForDuplication {
    fn parse(&self, path: &Path) -> Result<Box<ModuleTerm>, ParseError> {
        if path == Path::new("/file1.cds") {
            return Ok(Box::new(ModuleTerm::new(
                get_mock_location(),
                vec![
                    ModuleDefinition::Import(ImportTerm::new(
                        Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                        Box::new(KeywordTerm::new(
                            Location::new(0, 0, &Path::new("")),
                            Keyword::Using,
                        )),
                        Box::new(SelectionBlockTerm::new(
                            Location::new(
                                6,
                                11,
                                &Path::new("./tests/projects/modules/srv/books.cds"),
                            ),
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
                        Box::new(PathTerm::new(get_mock_location(), PathBuf::from("/file2"))),
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
        if path == Path::new("/file2.cds") {
            return Ok(Box::new(ModuleTerm::new(
                get_mock_location(),
                vec![ModuleDefinition::Service(ServiceDeclarationTerm::new(
                    get_mock_location(),
                    Box::new(KeywordTerm::new(get_mock_location(), Keyword::Service)),
                    Box::new(IdentifierTerm::new_basic(
                        get_mock_location(),
                        "AuthorsService1",
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
                ))],
            )));
        }
        if path == Path::new("/file2/index.cds") {
            return Ok(Box::new(ModuleTerm::new(
                get_mock_location(),
                vec![ModuleDefinition::Service(ServiceDeclarationTerm::new(
                    get_mock_location(),
                    Box::new(KeywordTerm::new(get_mock_location(), Keyword::Service)),
                    Box::new(IdentifierTerm::new_basic(
                        get_mock_location(),
                        "AuthorsService2",
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
                ))],
            )));
        }
        Err(ParseError::new(
            ErrorCode::FileIOError,
            "Unexpected file".to_string(),
        ))
    }
}

#[test]
fn test_parse_duplication() {
    let mut directories = HashMap::new();
    directories.insert(
        PathBuf::from("/"),
        vec![PathBuf::from("/file1.cds"), PathBuf::from("/file2.cds")],
    );
    directories.insert(
        PathBuf::from("/file2/"),
        vec![PathBuf::from("/file2/index.cds")],
    );

    let mut files = HashMap::new();
    files.insert(PathBuf::from("/file1.cds"), "".to_string());
    files.insert(PathBuf::from("/file2.cds"), "".to_string());
    files.insert(PathBuf::from("/file2/index.cds"), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParserForDuplication);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec![get_file_1_path()]);

    assert!(result.is_err());
    let parse_error = result.err().unwrap();
    assert_eq!(parse_error.error_code(), ErrorCode::LinkingError);
}
