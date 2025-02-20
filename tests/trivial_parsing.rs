use std::path::Path;

use open_cds_parser::ast::*;
use open_cds_parser::parser::Parser;

#[test]
fn with_correct_and_trivial_cds_it_buids_a_tree() {
    let parser = Parser::new_with_native_fs();

    let path = Path::new("./tests/projects/trivial/srv/cat-service.cds");
    let ast = parser.parse_single_module(path).unwrap();

    let expected_ast = Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
        ServiceTerm::new(
            Box::new(NameTerm::new("CatalogService".to_string())),
            vec![
                ServiceDefinition::Entity(EntityTerm::new(
                    Box::new(NameTerm::new("UserScopes".to_string())),
                    vec![],
                    vec![
                        FieldTerm::new(
                            Box::new(NameTerm::new("username".to_string())),
                            Box::new(NameTerm::new("String".to_string())),
                        ),
                        FieldTerm::new(
                            Box::new(NameTerm::new("scope".to_string())),
                            Box::new(NameTerm::new("String".to_string())),
                        ),
                    ],
                )),
                ServiceDefinition::Function(FunctionDeclarationTerm::new(
                    Box::new(FunctionTerm::new(Location::new(106, 114, path))),
                    Box::new(NameTerm::new("getUserScopesCount".to_string())),
                    Box::new(ParametersBlockTerm::new(
                        Location::new(133, 135, path),
                        Box::new(OpenRoundBracketTerm::new(Location::new(133, 134, path))),
                        vec![],
                        Box::new(CloseRoundBracketTerm::new(Location::new(134, 135, path))),
                    )),
                    Box::new(TypeReferenceTerm::new_scalar(Box::new(NameTerm::new(
                        "Integer".to_string(),
                    )))),
                )),
            ],
        ),
    )]));
    assert_eq!(ast, expected_ast);
}
