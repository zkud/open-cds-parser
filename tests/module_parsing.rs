use std::collections::HashMap;
use std::path::Path;

use open_cds_parser::ast::*;
use open_cds_parser::parser::Parser;

#[test]
fn with_correct_and_multi_module_it_builds_modules_table() {
    let parser = Parser::new_with_native_fs();

    let ast = parser
        .parse_multiple_module(vec!["./tests/projects/modules/srv".to_string()])
        .unwrap();

    let mut expected_ast = HashMap::new();
    let schema_path = Path::new("./tests/projects/modules/db/schema.cds")
        .canonicalize()
        .unwrap();
    expected_ast.insert(
        schema_path.to_string_lossy().to_string(),
        ModuleTerm::new(vec![ModuleDefinition::Entity(EntityTerm::new(
            Box::new(NameTerm::new("Books".to_string())),
            vec![],
            vec![
                FieldTerm::new(
                    Box::new(NameTerm::new("id".to_string())),
                    Box::new(NameTerm::new("UUID".to_string())),
                ),
                FieldTerm::new(
                    Box::new(NameTerm::new("name".to_string())),
                    Box::new(NameTerm::new("String".to_string())),
                ),
            ],
        ))]),
    );
    let service_path = Path::new("./tests/projects/modules/srv/books.cds")
        .canonicalize()
        .unwrap();
    expected_ast.insert(
        service_path.to_string_lossy().to_string(),
        ModuleTerm::new(vec![
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
                Box::new(PathTerm::new("../db/schema".to_string())),
                Box::new(SemicolumnTerm::new()),
            )),
            ModuleDefinition::Service(ServiceTerm::new(
                Box::new(NameTerm::new("BooksService".to_string())),
                vec![],
            )),
        ]),
    );
    assert_eq!(ast, expected_ast);
}
