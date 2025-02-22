use std::collections::HashMap;
use std::path::{Path, PathBuf};

use open_cds_parser::ast::*;
use open_cds_parser::parser::Parser;

#[test]
fn with_correct_and_multi_module_it_builds_modules_table() {
    let parser = Parser::new_with_native_fs();

    let ast = parser
        .parse_multiple_modules(vec![PathBuf::from("./tests/projects/modules/srv")])
        .unwrap();

    let mut expected_ast = HashMap::new();
    let schema_path = PathBuf::from("./tests/projects/modules/db/schema.cds")
        .canonicalize()
        .unwrap();
    expected_ast.insert(
        schema_path,
        ModuleTerm::new(vec![ModuleDefinition::Entity(EntityTerm::new(
            Box::new(IdentifierTerm::new("Books".to_string())),
            vec![],
            vec![
                FieldTerm::new(
                    Box::new(IdentifierTerm::new("id".to_string())),
                    Box::new(IdentifierTerm::new("UUID".to_string())),
                ),
                FieldTerm::new(
                    Box::new(IdentifierTerm::new("name".to_string())),
                    Box::new(IdentifierTerm::new("String".to_string())),
                ),
            ],
        ))]),
    );
    let service_path = PathBuf::from("./tests/projects/modules/srv/books.cds")
        .canonicalize()
        .unwrap();
    expected_ast.insert(
        service_path,
        ModuleTerm::new(vec![
            ModuleDefinition::Import(ImportTerm::new(
                Location::new(0, 32, &Path::new("./tests/projects/modules/srv/books.cds")),
                Box::new(UsingTerm::new(Location::new(
                    0,
                    5,
                    &Path::new("./tests/projects/modules/srv/books.cds"),
                ))),
                Box::new(SelectionBlockTerm::new(
                    Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                    None,
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Location::new(6, 11, &Path::new("./tests/projects/modules/srv/books.cds")),
                        Box::new(ImportIdentifierTerm::new(
                            Location::new(
                                6,
                                11,
                                &Path::new("./tests/projects/modules/srv/books.cds"),
                            ),
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
                Box::new(PathTerm::new(PathBuf::from("../db/schema"))),
                Box::new(SemicolumnTerm::new(Location::new(
                    31,
                    32,
                    &Path::new("./tests/projects/modules/srv/books.cds"),
                ))),
            )),
            ModuleDefinition::Service(ServiceTerm::new(
                Box::new(IdentifierTerm::new("BooksService".to_string())),
                vec![],
            )),
        ]),
    );
    assert_eq!(ast, expected_ast);
}
