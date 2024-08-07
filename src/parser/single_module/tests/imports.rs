use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn with_straight_wildcart_import_it_parses() {
    let mut files = HashMap::new();
    files.insert(
        "/import.cds".to_string(),
        "
        using * from 'path';
    "
        .to_string(),
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
