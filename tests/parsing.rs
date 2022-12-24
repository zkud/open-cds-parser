use open_cds_ast::ast::{
  EntityTerm, FieldTerm, ModuleDefinition, ModuleTerm, NameTerm, ServiceDefinition, ServiceTerm,
};
use open_cds_ast::parser::Parser;

#[test]
fn with_correct_and_trivial_cds_it_buids_a_tree() {
  let parser = Parser::new("./tests/projects/trivial/srv/cat-service.cds".to_string());

  let ast = parser.parse().unwrap();

  let expected_ast = Box::new(ModuleTerm::new(vec![ModuleDefinition::Service(
    ServiceTerm::new(
      Box::new(NameTerm::new("CatalogService".to_string())),
      vec![ServiceDefinition::Entity(EntityTerm::new(
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
      ))],
    ),
  )]));
  assert_eq!(ast, expected_ast);
}
