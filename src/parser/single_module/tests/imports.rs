use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/**
 * doing a little trick, as Path/PathBuf cannot be a constant / will be copied anyways
 */
#[inline]
fn get_import_path() -> PathBuf {
    PathBuf::from("/import.cds")
}

#[test]
fn with_straight_wildcart_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(get_import_path(), "using * from 'path';".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse(&get_import_path());

    expect_import_to_be(
        result,
        ImportTerm::new(
            Location::new(0, 20, &get_import_path()),
            Box::new(KeywordTerm::new(
                Location::new(0, 5, &get_import_path()),
                Keyword::Using,
            )),
            Box::new(SelectionBlockTerm::new(
                Location::new(6, 7, &get_import_path()),
                None,
                vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                    Location::new(6, 7, &get_import_path()),
                    Box::new(ImportIdentifierTerm::new(
                        Location::new(6, 7, &get_import_path()),
                        Box::new(ImportIdentifierVariant::SelectAll(Box::new(
                            PunctuationSignTerm::new(
                                Location::new(6, 7, &get_import_path()),
                                PunctuationSign::Wildcart,
                            ),
                        ))),
                    )),
                    None,
                    None,
                ))],
                None,
            )),
            Box::new(KeywordTerm::new(
                Location::new(8, 12, &get_import_path()),
                Keyword::From,
            )),
            Box::new(PathTerm::new(
                Location::new(13, 19, &get_import_path()),
                PathBuf::from("path"),
            )),
            Box::new(PunctuationSignTerm::new(
                Location::new(19, 20, &get_import_path()),
                PunctuationSign::Semicolumn,
            )),
        ),
    );
}

#[inline]
fn expect_import_to_be(
    parsed_module: Result<Box<ModuleTerm>, ParseError>,
    expected_import: ImportTerm,
) {
    let module = parsed_module.expect("Parsing is failing");
    assert_eq!(module.definitions().len(), 1);
    let import = module
        .definitions()
        .get(0)
        .expect("Unable to retrieve import");
    let import = if let ModuleDefinition::Import(import) = import {
        import
    } else {
        panic!("Missed import")
    };
    assert_eq!(import, &expected_import);
}

#[test]
fn with_straigh_wildcart_import_with_braces_it_parses() {
    let mut files = HashMap::new();
    files.insert(get_import_path(), "using { * } from 'path';".to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse(&get_import_path());

    expect_import_to_be(
        result,
        ImportTerm::new(
            Location::new(0, 24, &get_import_path()),
            Box::new(KeywordTerm::new(
                Location::new(0, 5, &get_import_path()),
                Keyword::Using,
            )),
            Box::new(SelectionBlockTerm::new(
                Location::new(6, 11, &get_import_path()),
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(6, 7, &get_import_path()),
                    PunctuationSign::OpenCurlyBrace,
                ))),
                vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                    Location::new(8, 9, &get_import_path()),
                    Box::new(ImportIdentifierTerm::new(
                        Location::new(8, 9, &get_import_path()),
                        Box::new(ImportIdentifierVariant::SelectAll(Box::new(
                            PunctuationSignTerm::new(
                                Location::new(8, 9, &get_import_path()),
                                PunctuationSign::Wildcart,
                            ),
                        ))),
                    )),
                    None,
                    None,
                ))],
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(10, 11, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                ))),
            )),
            Box::new(KeywordTerm::new(
                Location::new(12, 16, &get_import_path()),
                Keyword::From,
            )),
            Box::new(PathTerm::new(
                Location::new(17, 23, &get_import_path()),
                PathBuf::from("path"),
            )),
            Box::new(PunctuationSignTerm::new(
                Location::new(23, 24, &get_import_path()),
                PunctuationSign::Semicolumn,
            )),
        ),
    );
}

#[test]
fn with_name_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        get_import_path(),
        " using { name } from 'path'; ".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse(&get_import_path());

    expect_import_to_be(
        result,
        ImportTerm::new(
            Location::new(1, 28, &get_import_path()),
            Box::new(KeywordTerm::new(
                Location::new(1, 6, &get_import_path()),
                Keyword::Using,
            )),
            Box::new(SelectionBlockTerm::new(
                Location::new(7, 15, &get_import_path()),
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(7, 8, &get_import_path()),
                    PunctuationSign::OpenCurlyBrace,
                ))),
                vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                    Location::new(9, 13, &get_import_path()),
                    Box::new(ImportIdentifierTerm::new(
                        Location::new(9, 13, &get_import_path()),
                        Box::new(ImportIdentifierVariant::IdentifierOnly(Box::new(
                            IdentifierTerm::new_basic(
                                Location::new(9, 13, &get_import_path()),
                                "name",
                            ),
                        ))),
                    )),
                    None,
                    None,
                ))],
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(14, 15, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                ))),
            )),
            Box::new(KeywordTerm::new(
                Location::new(16, 20, &get_import_path()),
                Keyword::From,
            )),
            Box::new(PathTerm::new(
                Location::new(21, 27, &get_import_path()),
                PathBuf::from("path"),
            )),
            Box::new(PunctuationSignTerm::new(
                Location::new(27, 28, &get_import_path()),
                PunctuationSign::Semicolumn,
            )),
        ),
    );
}

#[test]
fn with_name_with_comma_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        get_import_path(),
        " using { name, } from 'path'; ".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse(&get_import_path());

    expect_import_to_be(
        result,
        ImportTerm::new(
            Location::new(1, 29, &get_import_path()),
            Box::new(KeywordTerm::new(
                Location::new(1, 6, &get_import_path()),
                Keyword::Using,
            )),
            Box::new(SelectionBlockTerm::new(
                Location::new(7, 16, &get_import_path()),
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(7, 8, &get_import_path()),
                    PunctuationSign::OpenCurlyBrace,
                ))),
                vec![
                    SelectionBlockSegment::Selector(SelectorTerm::new(
                        Location::new(9, 13, &get_import_path()),
                        Box::new(ImportIdentifierTerm::new(
                            Location::new(9, 13, &get_import_path()),
                            Box::new(ImportIdentifierVariant::IdentifierOnly(Box::new(
                                IdentifierTerm::new_basic(
                                    Location::new(9, 13, &get_import_path()),
                                    "name",
                                ),
                            ))),
                        )),
                        None,
                        None,
                    )),
                    SelectionBlockSegment::Comma(PunctuationSignTerm::new(
                        Location::new(13, 14, &get_import_path()),
                        PunctuationSign::Comma,
                    )),
                ],
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(15, 16, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                ))),
            )),
            Box::new(KeywordTerm::new(
                Location::new(17, 21, &get_import_path()),
                Keyword::From,
            )),
            Box::new(PathTerm::new(
                Location::new(22, 28, &get_import_path()),
                PathBuf::from("path"),
            )),
            Box::new(PunctuationSignTerm::new(
                Location::new(28, 29, &get_import_path()),
                PunctuationSign::Semicolumn,
            )),
        ),
    );
}

#[test]
fn with_name_with_alias_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        get_import_path(),
        " using { name as name2 } from 'path'; ".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse(&get_import_path());
    expect_import_to_be(
        result,
        ImportTerm::new(
            Location::new(1, 37, &get_import_path()),
            Box::new(KeywordTerm::new(
                Location::new(1, 6, &get_import_path()),
                Keyword::Using,
            )),
            Box::new(SelectionBlockTerm::new(
                Location::new(7, 24, &get_import_path()),
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(7, 8, &get_import_path()),
                    PunctuationSign::OpenCurlyBrace,
                ))),
                vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                    Location::new(9, 22, &get_import_path()),
                    Box::new(ImportIdentifierTerm::new(
                        Location::new(9, 13, &get_import_path()),
                        Box::new(ImportIdentifierVariant::IdentifierOnly(Box::new(
                            IdentifierTerm::new_basic(
                                Location::new(9, 13, &get_import_path()),
                                "name",
                            ),
                        ))),
                    )),
                    Some(Box::new(KeywordTerm::new(
                        Location::new(14, 16, &get_import_path()),
                        Keyword::As,
                    ))),
                    Some(Box::new(IdentifierTerm::new_basic(
                        Location::new(17, 22, &get_import_path()),
                        "name2",
                    ))),
                ))],
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(23, 24, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                ))),
            )),
            Box::new(KeywordTerm::new(
                Location::new(25, 29, &get_import_path()),
                Keyword::From,
            )),
            Box::new(PathTerm::new(
                Location::new(30, 36, &get_import_path()),
                PathBuf::from("path"),
            )),
            Box::new(PunctuationSignTerm::new(
                Location::new(36, 37, &get_import_path()),
                PunctuationSign::Semicolumn,
            )),
        ),
    );
}

#[test]
fn with_name_with_wildcart_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        get_import_path(),
        " using { name.* } from 'path'; ".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse(&get_import_path());

    expect_import_to_be(
        result,
        ImportTerm::new(
            Location::new(1, 30, &get_import_path()),
            Box::new(KeywordTerm::new(
                Location::new(1, 6, &get_import_path()),
                Keyword::Using,
            )),
            Box::new(SelectionBlockTerm::new(
                Location::new(7, 17, &get_import_path()),
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(7, 8, &get_import_path()),
                    PunctuationSign::OpenCurlyBrace,
                ))),
                vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                    Location::new(9, 15, &get_import_path()),
                    Box::new(ImportIdentifierTerm::new(
                        Location::new(9, 15, &get_import_path()),
                        Box::new(ImportIdentifierVariant::IdentifierWithWildcart {
                            identifier: Box::new(IdentifierTerm::new_basic(
                                Location::new(9, 13, &get_import_path()),
                                "name",
                            )),
                            dot: Box::new(PunctuationSignTerm::new(
                                Location::new(13, 14, &get_import_path()),
                                PunctuationSign::Dot,
                            )),
                            wildcart: Box::new(PunctuationSignTerm::new(
                                Location::new(14, 15, &get_import_path()),
                                PunctuationSign::Wildcart,
                            )),
                        }),
                    )),
                    None,
                    None,
                ))],
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(16, 17, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                ))),
            )),
            Box::new(KeywordTerm::new(
                Location::new(18, 22, &get_import_path()),
                Keyword::From,
            )),
            Box::new(PathTerm::new(
                Location::new(23, 29, &get_import_path()),
                PathBuf::from("path"),
            )),
            Box::new(PunctuationSignTerm::new(
                Location::new(29, 30, &get_import_path()),
                PunctuationSign::Semicolumn,
            )),
        ),
    );
}

#[test]
fn with_multiple_imports_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        get_import_path(),
        " using { name1, name2 } from 'path'; ".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse(&get_import_path());

    expect_import_to_be(
        result,
        ImportTerm::new(
            Location::new(1, 36, &get_import_path()),
            Box::new(KeywordTerm::new(
                Location::new(1, 6, &get_import_path()),
                Keyword::Using,
            )),
            Box::new(SelectionBlockTerm::new(
                Location::new(7, 23, &get_import_path()),
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(7, 8, &get_import_path()),
                    PunctuationSign::OpenCurlyBrace,
                ))),
                vec![
                    SelectionBlockSegment::Selector(SelectorTerm::new(
                        Location::new(9, 14, &get_import_path()),
                        Box::new(ImportIdentifierTerm::new(
                            Location::new(9, 14, &get_import_path()),
                            Box::new(ImportIdentifierVariant::IdentifierOnly(Box::new(
                                IdentifierTerm::new_basic(
                                    Location::new(9, 14, &get_import_path()),
                                    "name1",
                                ),
                            ))),
                        )),
                        None,
                        None,
                    )),
                    SelectionBlockSegment::Comma(PunctuationSignTerm::new(
                        Location::new(14, 15, &get_import_path()),
                        PunctuationSign::Comma,
                    )),
                    SelectionBlockSegment::Selector(SelectorTerm::new(
                        Location::new(16, 21, &get_import_path()),
                        Box::new(ImportIdentifierTerm::new(
                            Location::new(16, 21, &get_import_path()),
                            Box::new(ImportIdentifierVariant::IdentifierOnly(Box::new(
                                IdentifierTerm::new_basic(
                                    Location::new(16, 21, &get_import_path()),
                                    "name2",
                                ),
                            ))),
                        )),
                        None,
                        None,
                    )),
                ],
                Some(Box::new(PunctuationSignTerm::new(
                    Location::new(22, 23, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                ))),
            )),
            Box::new(KeywordTerm::new(
                Location::new(24, 28, &get_import_path()),
                Keyword::From,
            )),
            Box::new(PathTerm::new(
                Location::new(29, 35, &get_import_path()),
                PathBuf::from("path"),
            )),
            Box::new(PunctuationSignTerm::new(
                Location::new(35, 36, &get_import_path()),
                PunctuationSign::Semicolumn,
            )),
        ),
    );
}

struct EmtpyVisitor;

impl Visitor for EmtpyVisitor {
    type Error = ();

    fn process<T: ASTTerm>(&mut self, _: &T) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn with_various_scenarios_it_visits_successfully() {
    let mut files = HashMap::new();
    files.insert(
        get_import_path(),
        "
        using { name1, name2 } from 'path';
        using { name3 as name4, name5 as name6 } from 'path2';
        using { name3 as name4, name5 as name6 } from 'path3';
        using { * } from 'path4';
        using { name7.* } from 'path5';
        "
        .to_string(),
    );
    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);
    let mut visitor = EmtpyVisitor {};

    let result = parser.parse(&get_import_path());
    let parsed_module = result.unwrap();
    let visit_result = parsed_module.accept(&mut visitor);

    assert!(visit_result.is_ok());
}
