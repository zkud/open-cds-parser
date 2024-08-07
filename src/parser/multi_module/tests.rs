use super::*;
use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::parse_error::ParseError;
use crate::parser::single_module::SingleModuleParser;
use crate::parser::ParseErrorType;
use std::collections::HashMap;
use std::sync::Arc;

struct MockSingleModuleParser;

impl SingleModuleParser for MockSingleModuleParser {
    fn parse(&self, path: &str) -> Result<Box<ModuleTerm>, ParseError> {
        match path {
            "/file1.cds" => Ok(Box::new(ModuleTerm::new(vec![
                ModuleDefinition::Import(ImportTerm::new(
                    Box::new(UsingTerm::new()),
                    Box::new(SelectionBlockTerm::new(
                        None,
                        vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                            Box::new(ImportIdentifierTerm::new(Box::new(
                                ImportIdentifierVariant::NameOnly(Box::new(NameTerm::new(
                                    "Books".to_string(),
                                ))),
                            ))),
                            None,
                            None,
                        ))],
                        None,
                    )),
                    Box::new(FromTerm::new()),
                    Box::new(PathTerm::new("/subdir/file3".to_string())),
                    Box::new(SemicolumnTerm::new()),
                )),
                ModuleDefinition::Service(ServiceTerm::new(
                    Box::new(NameTerm::new("BooksService".to_string())),
                    vec![],
                )),
            ]))),
            "/file2.cds" => Ok(Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
                ServiceTerm::new(
                    Box::new(NameTerm::new("AuthorsService1".to_string())),
                    vec![],
                ),
            )]))),
            "/subdir/file3.cds" => Ok(Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
                ServiceTerm::new(
                    Box::new(NameTerm::new("AuthorsService2".to_string())),
                    vec![],
                ),
            )]))),
            _ => Err(ParseError::new(
                "Unexpected file".to_string(),
                ParseErrorType::FileIOError,
            )),
        }
    }
}

#[test]
fn test_parse_single_file() {
    let mut directories = HashMap::new();
    directories.insert("/".to_string(), vec!["/file2.cds".to_string()]);

    let mut files = HashMap::new();
    files.insert("/file2.cds".to_string(), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Box::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec!["/file2.cds".to_string()]);

    assert!(result.is_ok());
    let parsed_modules = result.unwrap();
    assert_eq!(parsed_modules.len(), 1);
    assert!(parsed_modules.contains_key("/file2.cds"));
}

#[test]
fn test_parse_directory() {
    let mut directories = HashMap::new();
    directories.insert(
        "/".to_string(),
        vec!["/file1.cds".to_string(), "/file2.cds".to_string()],
    );
    directories.insert(
        "/subdir/".to_string(),
        vec!["/subdir/file3.cds".to_string()],
    );

    let mut files = HashMap::new();
    files.insert("/file1.cds".to_string(), "".to_string());
    files.insert("/file2.cds".to_string(), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Box::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec!["/".to_string()]);

    assert!(result.is_ok());
    let parsed_modules = result.unwrap();
    assert_eq!(parsed_modules.len(), 3);
    assert!(parsed_modules.contains_key("/file1.cds"));
    assert!(parsed_modules.contains_key("/file2.cds"));
}

#[test]
fn test_parse_invalid_path() {
    let directories = HashMap::new();
    let files = HashMap::new();

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Box::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec!["invalid_path".to_string()]);

    assert!(result.is_err());
}

#[test]
fn test_parse_with_imports() {
    let mut directories = HashMap::new();
    directories.insert("/".to_string(), vec!["/file1.cds".to_string()]);
    directories.insert(
        "/subdir/".to_string(),
        vec!["/subdir/file3.cds".to_string()],
    );

    let mut files = HashMap::new();
    files.insert("/file1.cds".to_string(), "".to_string());
    files.insert("/subdir/file3.cds".to_string(), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Box::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec!["/file1.cds".to_string()]);

    assert!(result.is_ok());
    let parsed_modules = result.unwrap();
    assert_eq!(parsed_modules.len(), 2);
    assert!(parsed_modules.contains_key("/file1.cds"));
    assert!(parsed_modules.contains_key("/subdir/file3.cds"));
}
