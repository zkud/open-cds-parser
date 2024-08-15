use open_cds_parser::ast::{
    EntityTerm, FieldTerm, FunctionTerm, ModuleDefinition, ModuleTerm, NameTerm, ReturnsTerm,
    ServiceDefinition, ServiceTerm,
};
use open_cds_parser::parser::Parser;

#[test]
fn with_correct_and_trivial_cds_it_buids_a_tree() {
    let parser = Parser::new_with_native_fs();

    let ast = parser
        .parse_single_module(&"./tests/projects/trivial/srv/cat-service.cds")
        .unwrap();

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
                ServiceDefinition::Function(FunctionTerm::new(
                    Box::new(NameTerm::new("getUserScopesCount".to_string())),
                    vec![],
                    Box::new(ReturnsTerm::new_scalar(Box::new(NameTerm::new(
                        "Integer".to_string(),
                    )))),
                )),
            ],
        ),
    )]));
    assert_eq!(ast, expected_ast);
}
