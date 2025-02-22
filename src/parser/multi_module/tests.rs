use super::*;
use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::parse_error::ParseError;
use crate::parser::single_module::SingleModuleParser;
use crate::parser::ParseErrorType;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[inline]
fn get_file_1_path() -> PathBuf {
    PathBuf::from("/file1.cds")
}

#[inline]
fn get_file_2_path() -> PathBuf {
    PathBuf::from("/file2.cds")
}

#[inline]
fn get_subdir_file_3_path() -> PathBuf {
    PathBuf::from("/subdir/file3.cds")
}

#[inline]
fn get_subdir_file_4_path() -> PathBuf {
    PathBuf::from("/subdir/file4.cds")
}

#[inline]
fn get_subdir_subdir_index_path() -> PathBuf {
    PathBuf::from("/subdir/subdir/index.cds")
}

#[inline]
fn get_failure_no_file_present_path() -> PathBuf {
    PathBuf::from("/failure_no_file_present.cds")
}

struct MockSingleModuleParser;

impl SingleModuleParser for MockSingleModuleParser {
    fn parse(&self, path: &Path) -> Result<Box<ModuleTerm>, ParseError> {
        if path == get_file_1_path() {
            return Ok(Box::new(ModuleTerm::new(vec![
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
                                    IdentifierTerm::new("Books".to_string()),
                                ))),
                            )),
                            None,
                            None,
                        ))],
                        None,
                    )),
                    Box::new(FromTerm::new()),
                    Box::new(PathTerm::new(PathBuf::from("/subdir/file3"))),
                    Box::new(SemicolumnTerm::new(Location::new(0, 0, &PathBuf::new()))),
                )),
                ModuleDefinition::Service(ServiceTerm::new(
                    Box::new(IdentifierTerm::new("BooksService".to_string())),
                    vec![],
                )),
            ])));
        }
        if path == get_file_2_path() {
            return Ok(Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
                ServiceTerm::new(
                    Box::new(IdentifierTerm::new("AuthorsService1".to_string())),
                    vec![],
                ),
            )])));
        }
        if path == get_subdir_file_3_path() {
            return Ok(Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
                ServiceTerm::new(
                    Box::new(IdentifierTerm::new("AuthorsService2".to_string())),
                    vec![],
                ),
            )])));
        }
        if path == get_subdir_file_4_path() {
            return Ok(Box::new(ModuleTerm::new(vec![
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
                                    IdentifierTerm::new("Books".to_string()),
                                ))),
                            )),
                            None,
                            None,
                        ))],
                        None,
                    )),
                    Box::new(FromTerm::new()),
                    Box::new(PathTerm::new(PathBuf::from("/subdir/subdir"))),
                    Box::new(SemicolumnTerm::new(Location::new(0, 0, &PathBuf::new()))),
                )),
                ModuleDefinition::Service(ServiceTerm::new(
                    Box::new(IdentifierTerm::new("BooksService".to_string())),
                    vec![],
                )),
            ])));
        }
        if path == get_subdir_subdir_index_path() {
            return Ok(Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
                ServiceTerm::new(
                    Box::new(IdentifierTerm::new("AuthorsService2".to_string())),
                    vec![],
                ),
            )])));
        }
        if path == get_failure_no_file_present_path() {
            return Ok(Box::new(ModuleTerm::new(vec![
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
                                    IdentifierTerm::new("Books".to_string()),
                                ))),
                            )),
                            None,
                            None,
                        ))],
                        None,
                    )),
                    Box::new(FromTerm::new()),
                    Box::new(PathTerm::new(PathBuf::from("/no_file_present"))),
                    Box::new(SemicolumnTerm::new(Location::new(0, 0, &PathBuf::new()))),
                )),
                ModuleDefinition::Service(ServiceTerm::new(
                    Box::new(IdentifierTerm::new("BooksService".to_string())),
                    vec![],
                )),
            ])));
        }
        return Err(ParseError::new(
            "Unexpected file".to_string(),
            ParseErrorType::FileIOError,
        ));
    }
}

#[test]
fn test_parse_single_file() {
    let mut directories = HashMap::new();
    directories.insert(PathBuf::from("/"), vec![PathBuf::from("/file2.cds")]);

    let mut files = HashMap::new();
    files.insert(PathBuf::from("/file2.cds"), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec![get_file_2_path()]);

    assert!(result.is_ok());
    let parsed_modules = result.unwrap();
    assert_eq!(parsed_modules.len(), 1);
    assert!(parsed_modules.contains_key(&get_file_2_path()));
}

#[test]
fn test_parse_directory() {
    let mut directories = HashMap::new();
    directories.insert(
        PathBuf::from("/"),
        vec![PathBuf::from("/file1.cds"), PathBuf::from("/file2.cds")],
    );
    directories.insert(
        PathBuf::from("/subdir/"),
        vec![PathBuf::from("/subdir/file3.cds")],
    );

    let mut files = HashMap::new();
    files.insert(PathBuf::from("/file1.cds"), "".to_string());
    files.insert(PathBuf::from("/file2.cds"), "".to_string());
    files.insert(PathBuf::from("/subdir/file3.cds"), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec![PathBuf::from("/")]);

    assert!(result.is_ok());
    let parsed_modules = result.unwrap();
    assert_eq!(parsed_modules.len(), 3);
    assert!(parsed_modules.contains_key(&get_file_1_path()));
    assert!(parsed_modules.contains_key(&get_file_2_path()));
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
fn test_parse_with_imports() {
    let mut directories = HashMap::new();
    directories.insert(PathBuf::from("/"), vec![PathBuf::from("/file1.cds")]);
    directories.insert(
        PathBuf::from("/subdir/"),
        vec![PathBuf::from("/subdir/file3.cds")],
    );

    let mut files = HashMap::new();
    files.insert(PathBuf::from("/file1.cds"), "".to_string());
    files.insert(PathBuf::from("/subdir/file3.cds"), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec![get_file_1_path()]);

    assert!(result.is_ok());
    let parsed_modules = result.unwrap();
    assert_eq!(parsed_modules.len(), 2);
    assert!(parsed_modules.contains_key(&get_file_1_path()));
    assert!(parsed_modules.contains_key(&get_subdir_file_3_path()));
}

#[test]
fn test_parse_with_imports_but_import_is_dir() {
    let mut directories = HashMap::new();
    directories.insert(
        PathBuf::from("/subdir/"),
        vec![PathBuf::from("/subdir/file4.cds")],
    );
    directories.insert(
        PathBuf::from("/subdir/subdir/"),
        vec![PathBuf::from("/subdir/subdir/index.cds")],
    );

    let mut files = HashMap::new();
    files.insert(PathBuf::from("/subdir/file4.cds"), "".to_string());
    files.insert(PathBuf::from("/subdir/subdir/index.cds"), "".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(directories, files));
    let single_module_parser = Arc::new(MockSingleModuleParser);

    let parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = parser.parse(vec![get_subdir_file_4_path()]);

    assert!(result.is_ok());
    let parsed_modules = result.unwrap();
    assert_eq!(parsed_modules.len(), 2);
    assert!(parsed_modules.contains_key(&get_subdir_file_4_path()));
    assert!(parsed_modules.contains_key(&get_subdir_subdir_index_path()));
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
    assert_eq!(parse_error.get_error_type(), ParseErrorType::FileIOError);
}

struct MockSingleModuleParserForDuplication;

impl SingleModuleParser for MockSingleModuleParserForDuplication {
    fn parse(&self, path: &Path) -> Result<Box<ModuleTerm>, ParseError> {
        if path == Path::new("/file1.cds") {
            return Ok(Box::new(ModuleTerm::new(vec![
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
                                    IdentifierTerm::new("Books".to_string()),
                                ))),
                            )),
                            None,
                            None,
                        ))],
                        None,
                    )),
                    Box::new(FromTerm::new()),
                    Box::new(PathTerm::new(PathBuf::from("/file2"))),
                    Box::new(SemicolumnTerm::new(Location::new(0, 0, &PathBuf::new()))),
                )),
                ModuleDefinition::Service(ServiceTerm::new(
                    Box::new(IdentifierTerm::new("BooksService".to_string())),
                    vec![],
                )),
            ])));
        }
        if path == Path::new("/file2.cds") {
            return Ok(Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
                ServiceTerm::new(
                    Box::new(IdentifierTerm::new("AuthorsService1".to_string())),
                    vec![],
                ),
            )])));
        }
        if path == Path::new("/file2/index.cds") {
            return Ok(Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
                ServiceTerm::new(
                    Box::new(IdentifierTerm::new("AuthorsService2".to_string())),
                    vec![],
                ),
            )])));
        }
        Err(ParseError::new(
            "Unexpected file".to_string(),
            ParseErrorType::FileIOError,
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
    assert_eq!(parse_error.get_error_type(), ParseErrorType::FileIOError);
}
