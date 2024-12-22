use super::*;
use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::parse_error::ParseError;
use crate::parser::single_module::SingleModuleParser;
use crate::parser::ParseErrorType;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

struct MockSingleModuleParser;

impl SingleModuleParser for MockSingleModuleParser {
    fn parse(&self, path: &str) -> Result<Box<ModuleTerm>, ParseError> {
        match path {
            "/file1.cds" => Ok(Box::new(ModuleTerm::new(vec![
                ModuleDefinition::Import(ImportTerm::new(
                    Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                    Box::new(UsingTerm::new(Location::new(0, 0, &PathBuf::new()))),
                    Box::new(SelectionBlockTerm::new(
                        Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                        None,
                        vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                            Location::new(
                                6,
                                11,
                                &Path::new("./tests/projects/modules/srv/books.cds"),
                            ),
                            Box::new(ImportIdentifierTerm::new(
                                Location::new(0, 0, &PathBuf::new()),
                                Box::new(ImportIdentifierVariant::NameOnly(Box::new(
                                    NameTerm::new("Books".to_string()),
                                ))),
                            )),
                            None,
                            None,
                        ))],
                        None,
                    )),
                    Box::new(FromTerm::new()),
                    Box::new(PathTerm::new("/subdir/file3".to_string())),
                    Box::new(SemicolumnTerm::new(Location::new(0, 0, &PathBuf::new()))),
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
            "/subdir/file4.cds" => Ok(Box::new(ModuleTerm::new(vec![
                ModuleDefinition::Import(ImportTerm::new(
                    Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                    Box::new(UsingTerm::new(Location::new(0, 0, &PathBuf::new()))),
                    Box::new(SelectionBlockTerm::new(
                        Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                        None,
                        vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                            Location::new(
                                6,
                                11,
                                &Path::new("./tests/projects/modules/srv/books.cds"),
                            ),
                            Box::new(ImportIdentifierTerm::new(
                                Location::new(0, 0, &PathBuf::new()),
                                Box::new(ImportIdentifierVariant::NameOnly(Box::new(
                                    NameTerm::new("Books".to_string()),
                                ))),
                            )),
                            None,
                            None,
                        ))],
                        None,
                    )),
                    Box::new(FromTerm::new()),
                    Box::new(PathTerm::new("/subdir/subdir".to_string())),
                    Box::new(SemicolumnTerm::new(Location::new(0, 0, &PathBuf::new()))),
                )),
                ModuleDefinition::Service(ServiceTerm::new(
                    Box::new(NameTerm::new("BooksService".to_string())),
                    vec![],
                )),
            ]))),
            "/subdir/subdir/index.cds" => {
                Ok(Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
                    ServiceTerm::new(
                        Box::new(NameTerm::new("AuthorsService2".to_string())),
                        vec![],
                    ),
                )])))
            }
            "/failure_no_file_present.cds" => Ok(Box::new(ModuleTerm::new(vec![
                ModuleDefinition::Import(ImportTerm::new(
                    Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                    Box::new(UsingTerm::new(Location::new(0, 0, &PathBuf::new()))),
                    Box::new(SelectionBlockTerm::new(
                        Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                        None,
                        vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                            Location::new(
                                6,
                                11,
                                &Path::new("./tests/projects/modules/srv/books.cds"),
                            ),
                            Box::new(ImportIdentifierTerm::new(
                                Location::new(0, 0, &PathBuf::new()),
                                Box::new(ImportIdentifierVariant::NameOnly(Box::new(
                                    NameTerm::new("Books".to_string()),
                                ))),
                            )),
                            None,
                            None,
                        ))],
                        None,
                    )),
                    Box::new(FromTerm::new()),
                    Box::new(PathTerm::new("/no_file_present".to_string())),
                    Box::new(SemicolumnTerm::new(Location::new(0, 0, &PathBuf::new()))),
                )),
                ModuleDefinition::Service(ServiceTerm::new(
                    Box::new(NameTerm::new("BooksService".to_string())),
                    vec![],
                )),
            ]))),
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
    let single_module_parser = Arc::new(MockSingleModuleParser);

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
    files.insert("/subdir/file3.cds".to_string(), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec!["/".to_string()]);

    assert!(result.is_ok());
    let parsed_modules = result.unwrap();
    println!("{:?}", parsed_modules.keys());
    assert_eq!(parsed_modules.len(), 3);
    assert!(parsed_modules.contains_key("/file1.cds"));
    assert!(parsed_modules.contains_key("/file2.cds"));
}

#[test]
fn test_parse_invalid_path() {
    let directories = HashMap::new();
    let files = HashMap::new();

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParser);

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
    let single_module_parser = Arc::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec!["/file1.cds".to_string()]);

    assert!(result.is_ok());
    let parsed_modules = result.unwrap();
    assert_eq!(parsed_modules.len(), 2);
    assert!(parsed_modules.contains_key("/file1.cds"));
    assert!(parsed_modules.contains_key("/subdir/file3.cds"));
}

#[test]
fn test_parse_with_imports_but_import_is_dir() {
    let mut directories = HashMap::new();
    directories.insert(
        "/subdir/".to_string(),
        vec!["/subdir/file4.cds".to_string()],
    );
    directories.insert(
        "/subdir/subdir/".to_string(),
        vec!["/subdir/subdir/index.cds".to_string()],
    );

    let mut files = HashMap::new();
    files.insert("/subdir/file4.cds".to_string(), "".to_string());
    files.insert("/subdir/subdir/index.cds".to_string(), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec!["/subdir/file4.cds".to_string()]);

    assert!(result.is_ok());
    let parsed_modules = result.unwrap();
    assert_eq!(parsed_modules.len(), 2);
    assert!(parsed_modules.contains_key("/subdir/file4.cds"));
    assert!(parsed_modules.contains_key("/subdir/subdir/index.cds"));
}

#[test]
fn test_parse_invalid_path_in_import() {
    let mut directories = HashMap::new();
    directories.insert(
        "/".to_string(),
        vec!["/failure_no_file_present.cds".to_string()],
    );

    let mut files = HashMap::new();
    files.insert("/failure_no_file_present.cds".to_string(), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec!["/failure_no_file_present.cds".to_string()]);

    assert!(result.is_err());
    let parse_error = result.err().unwrap();
    assert_eq!(parse_error.get_error_type(), ParseErrorType::FileIOError);
}

struct MockSingleModuleParserForDuplication;

impl SingleModuleParser for MockSingleModuleParserForDuplication {
    fn parse(&self, path: &str) -> Result<Box<ModuleTerm>, ParseError> {
        match path {
            "/file1.cds" => Ok(Box::new(ModuleTerm::new(vec![
                ModuleDefinition::Import(ImportTerm::new(
                    Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                    Box::new(UsingTerm::new(Location::new(0, 0, &Path::new("")))),
                    Box::new(SelectionBlockTerm::new(
                        Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                        None,
                        vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                            Location::new(
                                6,
                                11,
                                &Path::new("./tests/projects/modules/srv/books.cds"),
                            ),
                            Box::new(ImportIdentifierTerm::new(
                                Location::new(0, 0, &PathBuf::new()),
                                Box::new(ImportIdentifierVariant::NameOnly(Box::new(
                                    NameTerm::new("Books".to_string()),
                                ))),
                            )),
                            None,
                            None,
                        ))],
                        None,
                    )),
                    Box::new(FromTerm::new()),
                    Box::new(PathTerm::new("/file2".to_string())),
                    Box::new(SemicolumnTerm::new(Location::new(0, 0, &PathBuf::new()))),
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
            "/file2/index.cds" => Ok(Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
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
fn test_parse_duplication() {
    let mut directories = HashMap::new();
    directories.insert(
        "/".to_string(),
        vec!["/file1.cds".to_string(), "/file2.cds".to_string()],
    );
    directories.insert("/file2/".to_string(), vec!["/file2/index.cds".to_string()]);

    let mut files = HashMap::new();
    files.insert("/file1.cds".to_string(), "".to_string());
    files.insert("/file2.cds".to_string(), "".to_string());
    files.insert("/file2/index.cds".to_string(), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParserForDuplication);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec!["/file1.cds".to_string()]);

    assert!(result.is_err());
    let parse_error = result.err().unwrap();
    assert_eq!(parse_error.get_error_type(), ParseErrorType::FileIOError);
}
