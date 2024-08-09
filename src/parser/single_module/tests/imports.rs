use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::visitor::Visitor;
use std::collections::HashMap;
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
                Box::new(UsingTerm::new()),
                Box::new(SelectionBlockTerm::new(
                    None,
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Box::new(ImportIdentifierTerm::new(Box::new(
                            ImportIdentifierVariant::SelectAll(Box::new(WildcartTerm::new()))
                        ))),
                        None,
                        None
                    ))],
                    None
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new())
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
                Box::new(UsingTerm::new()),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new())),
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Box::new(ImportIdentifierTerm::new(Box::new(
                            ImportIdentifierVariant::SelectAll(Box::new(WildcartTerm::new()))
                        ))),
                        None,
                        None
                    ))],
                    Some(Box::new(CloseCurlyBraceTerm::new())),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new())
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
                Box::new(UsingTerm::new()),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new())),
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Box::new(ImportIdentifierTerm::new(Box::new(
                            ImportIdentifierVariant::NameOnly(Box::new(NameTerm::new(
                                "name".to_string()
                            )))
                        ))),
                        None,
                        None
                    ))],
                    Some(Box::new(CloseCurlyBraceTerm::new())),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new())
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
                Box::new(UsingTerm::new()),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new())),
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
                        SelectionBlockSegment::Comma(CommaTerm::new())
                    ],
                    Some(Box::new(CloseCurlyBraceTerm::new())),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new())
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
                Box::new(UsingTerm::new()),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new())),
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Box::new(ImportIdentifierTerm::new(Box::new(
                            ImportIdentifierVariant::NameOnly(Box::new(NameTerm::new(
                                "name".to_string()
                            )),)
                        ))),
                        Some(Box::new(AsTerm::new())),
                        Some(Box::new(NameTerm::new("name2".to_string())))
                    ))],
                    Some(Box::new(CloseCurlyBraceTerm::new())),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new())
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
                Box::new(UsingTerm::new()),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new())),
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Box::new(ImportIdentifierTerm::new(Box::new(
                            ImportIdentifierVariant::NameWithWildcart {
                                name: Box::new(NameTerm::new("name".to_string())),
                                dot: Box::new(DotTerm::new()),
                                wildcart: Box::new(WildcartTerm::new()),
                            }
                        ))),
                        None,
                        None
                    ))],
                    Some(Box::new(CloseCurlyBraceTerm::new())),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new())
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
                Box::new(UsingTerm::new()),
                Box::new(SelectionBlockTerm::new(
                    Some(Box::new(OpenCurlyBraceTerm::new())),
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
                        SelectionBlockSegment::Comma(CommaTerm::new()),
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
                    Some(Box::new(CloseCurlyBraceTerm::new())),
                )),
                Box::new(FromTerm::new()),
                Box::new(PathTerm::new("path".to_string())),
                Box::new(SemicolumnTerm::new())
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
