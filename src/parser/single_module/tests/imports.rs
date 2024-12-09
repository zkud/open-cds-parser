use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

#[test]
fn with_straight_wildcart_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        "/import.cds".to_string(),
        "using * from 'path';".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse("/import.cds");

    assert!(result.is_ok());
    let parsed_module = result.unwrap();
    assert_eq!(
        parsed_module,
        Box::new(ModuleTerm::new(vec![ModuleDefinition::Import(
            ImportTerm::new(
                Box::new(UsingTerm::new(Location::new(
                    0,
                    5,
                    &Path::new("/import.cds")
                ))),
                Box::new(SelectionBlockTerm::new(
                    None,
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Box::new(ImportIdentifierTerm::new(Box::new(
                            ImportIdentifierVariant::SelectAll(Box::new(WildcartTerm::new(
                                Location::new(6, 7, &Path::new("/import.cds"))
                            )))
                        ))),
                        None,
                        None
                    ))],
                    None
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new(Location::new(
                    19,
                    20,
                    &Path::new("/import.cds")
                )))
            )
        ),]))
    );
}

#[test]
fn with_straigh_wildcart_import_with_braces_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        "/import.cds".to_string(),
        "using { * } from 'path';".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse("/import.cds");

    assert!(result.is_ok());
    let parsed_module = result.unwrap();
    assert_eq!(
        parsed_module,
        Box::new(ModuleTerm::new(vec![ModuleDefinition::Import(
            ImportTerm::new(
                Box::new(UsingTerm::new(Location::new(
                    0,
                    5,
                    &Path::new("/import.cds")
                ))),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new(Location::new(
                        6,
                        7,
                        &Path::new("/import.cds")
                    )))),
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Box::new(ImportIdentifierTerm::new(Box::new(
                            ImportIdentifierVariant::SelectAll(Box::new(WildcartTerm::new(
                                Location::new(8, 9, &Path::new("/import.cds"))
                            )))
                        ))),
                        None,
                        None
                    ))],
                    Some(Box::new(CloseCurlyBraceTerm::new(Location::new(
                        10,
                        11,
                        &Path::new("/import.cds")
                    )))),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new(Location::new(
                    23,
                    24,
                    &Path::new("/import.cds")
                )))
            )
        ),]))
    );
}

#[test]
fn with_name_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        "/import.cds".to_string(),
        " using { name } from 'path'; ".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse("/import.cds");

    assert!(result.is_ok());
    let parsed_module = result.unwrap();
    assert_eq!(
        parsed_module,
        Box::new(ModuleTerm::new(vec![ModuleDefinition::Import(
            ImportTerm::new(
                Box::new(UsingTerm::new(Location::new(
                    1,
                    6,
                    &Path::new("/import.cds")
                ))),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new(Location::new(
                        7,
                        8,
                        &Path::new("/import.cds")
                    )))),
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Box::new(ImportIdentifierTerm::new(Box::new(
                            ImportIdentifierVariant::NameOnly(Box::new(NameTerm::new(
                                "name".to_string()
                            )))
                        ))),
                        None,
                        None
                    ))],
                    Some(Box::new(CloseCurlyBraceTerm::new(Location::new(
                        14,
                        15,
                        &Path::new("/import.cds")
                    )))),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new(Location::new(
                    27,
                    28,
                    &Path::new("/import.cds")
                )))
            )
        ),]))
    );
}

#[test]
fn with_name_with_comma_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        "/import.cds".to_string(),
        " using { name, } from 'path'; ".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse("/import.cds");

    assert!(result.is_ok());
    let parsed_module = result.unwrap();
    assert_eq!(
        parsed_module,
        Box::new(ModuleTerm::new(vec![ModuleDefinition::Import(
            ImportTerm::new(
                Box::new(UsingTerm::new(Location::new(
                    1,
                    6,
                    &Path::new("/import.cds")
                ))),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new(Location::new(
                        7,
                        8,
                        &Path::new("/import.cds")
                    )))),
                    vec![
                        SelectionBlockSegment::Selector(SelectorTerm::new(
                            Box::new(ImportIdentifierTerm::new(Box::new(
                                ImportIdentifierVariant::NameOnly(Box::new(NameTerm::new(
                                    "name".to_string()
                                )))
                            ))),
                            None,
                            None
                        )),
                        SelectionBlockSegment::Comma(CommaTerm::new(Location::new(
                            13,
                            14,
                            &Path::new("/import.cds")
                        )))
                    ],
                    Some(Box::new(CloseCurlyBraceTerm::new(Location::new(
                        15,
                        16,
                        &Path::new("/import.cds")
                    )))),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new(Location::new(
                    28,
                    29,
                    &Path::new("/import.cds")
                )))
            )
        ),]))
    );
}

#[test]
fn with_name_with_alias_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        "/import.cds".to_string(),
        " using { name as name2 } from 'path'; ".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse("/import.cds");

    assert!(result.is_ok());
    let parsed_module = result.unwrap();
    assert_eq!(
        parsed_module,
        Box::new(ModuleTerm::new(vec![ModuleDefinition::Import(
            ImportTerm::new(
                Box::new(UsingTerm::new(Location::new(
                    1,
                    6,
                    &Path::new("/import.cds")
                ))),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new(Location::new(
                        7,
                        8,
                        &Path::new("/import.cds")
                    )))),
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Box::new(ImportIdentifierTerm::new(Box::new(
                            ImportIdentifierVariant::NameOnly(Box::new(NameTerm::new(
                                "name".to_string()
                            )),)
                        ))),
                        Some(Box::new(AsTerm::new(Location::new(
                            14,
                            16,
                            &Path::new("/import.cds")
                        )))),
                        Some(Box::new(NameTerm::new("name2".to_string())))
                    ))],
                    Some(Box::new(CloseCurlyBraceTerm::new(Location::new(
                        23,
                        24,
                        &Path::new("/import.cds")
                    )))),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new(Location::new(
                    36,
                    37,
                    &Path::new("/import.cds")
                )))
            )
        ),]))
    );
}

#[test]
fn with_name_with_wildcart_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        "/import.cds".to_string(),
        " using { name.* } from 'path'; ".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse("/import.cds");

    assert!(result.is_ok());
    let parsed_module = result.unwrap();
    assert_eq!(
        parsed_module,
        Box::new(ModuleTerm::new(vec![ModuleDefinition::Import(
            ImportTerm::new(
                Box::new(UsingTerm::new(Location::new(
                    1,
                    6,
                    &Path::new("/import.cds")
                ))),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new(Location::new(
                        7,
                        8,
                        &Path::new("/import.cds")
                    )))),
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Box::new(ImportIdentifierTerm::new(Box::new(
                            ImportIdentifierVariant::NameWithWildcart {
                                name: Box::new(NameTerm::new("name".to_string())),
                                dot: Box::new(DotTerm::new(Location::new(
                                    13,
                                    14,
                                    &Path::new("/import.cds")
                                ))),
                                wildcart: Box::new(WildcartTerm::new(Location::new(
                                    14,
                                    15,
                                    &Path::new("/import.cds")
                                ))),
                            }
                        ))),
                        None,
                        None
                    ))],
                    Some(Box::new(CloseCurlyBraceTerm::new(Location::new(
                        16,
                        17,
                        &Path::new("/import.cds")
                    )))),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new(Location::new(
                    29,
                    30,
                    &Path::new("/import.cds")
                )))
            )
        ),]))
    );
}

#[test]
fn with_multiple_imports_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        "/import.cds".to_string(),
        " using { name1, name2 } from 'path'; ".to_string(),
    );

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    let result = parser.parse("/import.cds");

    assert!(result.is_ok());
    let parsed_module = result.unwrap();
    assert_eq!(
        parsed_module,
        Box::new(ModuleTerm::new(vec![ModuleDefinition::Import(
            ImportTerm::new(
                Box::new(UsingTerm::new(Location::new(
                    1,
                    6,
                    &Path::new("/import.cds")
                ))),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new(Location::new(
                        7,
                        8,
                        &Path::new("/import.cds")
                    )))),
                    vec![
                        SelectionBlockSegment::Selector(SelectorTerm::new(
                            Box::new(ImportIdentifierTerm::new(Box::new(
                                ImportIdentifierVariant::NameOnly(Box::new(NameTerm::new(
                                    "name1".to_string()
                                )))
                            ))),
                            None,
                            None
                        )),
                        SelectionBlockSegment::Comma(CommaTerm::new(Location::new(
                            14,
                            15,
                            &Path::new("/import.cds")
                        ))),
                        SelectionBlockSegment::Selector(SelectorTerm::new(
                            Box::new(ImportIdentifierTerm::new(Box::new(
                                ImportIdentifierVariant::NameOnly(Box::new(NameTerm::new(
                                    "name2".to_string()
                                )))
                            ))),
                            None,
                            None
                        ))
                    ],
                    Some(Box::new(CloseCurlyBraceTerm::new(Location::new(
                        22,
                        23,
                        &Path::new("/import.cds")
                    )))),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new(Location::new(
                    35,
                    36,
                    &Path::new("/import.cds")
                )))
            )
        ),]))
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
        "/import.cds".to_string(),
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

    let result = parser.parse("/import.cds");
    let parsed_module = result.unwrap();
    let visit_result = parsed_module.accept(&mut visitor);

    assert!(visit_result.is_ok());
}
