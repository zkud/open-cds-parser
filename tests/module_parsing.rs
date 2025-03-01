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
        schema_path.clone(),
        ModuleTerm::new(vec![ModuleDefinition::Entity(EntityTerm::new(
            Box::new(IdentifierTerm::new_basic(
                Location::new(7, 12, &schema_path),
                "Books",
            )),
            vec![],
            vec![
                FieldTerm::new(
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(20, 22, &schema_path),
                        "id",
                    )),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(24, 28, &schema_path),
                        "UUID",
                    )),
                ),
                FieldTerm::new(
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(35, 39, &schema_path),
                        "name",
                    )),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(41, 47, &schema_path),
                        "String",
                    )),
                ),
            ],
        ))]),
    );
    let service_path = PathBuf::from("./tests/projects/modules/srv/books.cds")
        .canonicalize()
        .unwrap();
    expected_ast.insert(
        service_path.clone(),
        ModuleTerm::new(vec![
            ModuleDefinition::Import(ImportTerm::new(
                Location::new(0, 32, &service_path),
                Box::new(UsingTerm::new(Location::new(
                    0,
                    5,
                    &service_path,
                ))),
                Box::new(SelectionBlockTerm::new(
                    Location::new(6, 11, &service_path),
                    None,
                    vec![SelectionBlockSegment::Selector(SelectorTerm::new(
                        Location::new(6, 11, &service_path),
                        Box::new(ImportIdentifierTerm::new(
                            Location::new(
                                6,
                                11,
                                &service_path,
                            ),
                            Box::new(ImportIdentifierVariant::NameOnly(Box::new(
                                IdentifierTerm::new_basic(
                                    Location::new(6, 11, &service_path),
                                    "Books",
                                ),
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
                    &service_path,
                ))),
            )),
            ModuleDefinition::Service(ServiceTerm::new(
                Box::new(IdentifierTerm::new_basic(
                    Location::new(44, 56, &service_path),
                    "BooksService",
                )),
                vec![],
            )),
        ]),
    );
    assert_eq!(ast, expected_ast);
}
