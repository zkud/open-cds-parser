use open_cds_ast::ast::{
  EntityTerm, FieldTerm, ModuleDefinition, ModuleTerm, NameTerm, ServiceDefinition, ServiceTerm,
};
use open_cds_ast::parser::Parser;

#[test]
fn with_correct_and_trivial_cds_it_buids_a_tree() {
  let parser = Parser::new("./tests/projects/trivial/srv/cat-service.cds".to_string());

  let ast = parser.parse().unwrap();

  let expected_ast =
    ModuleTerm::new_boxed(vec![ModuleDefinition::Service(ServiceTerm::new_boxed(
      NameTerm::new_boxed("CatalogService".to_string()),
      vec![ServiceDefinition::Entity(EntityTerm::new_boxed(
        NameTerm::new_boxed("UserScopes".to_string()),
        vec![],
        vec![
          FieldTerm::new_boxed(
            NameTerm::new_boxed("username".to_string()),
            NameTerm::new_boxed("String".to_string()),
          ),
          FieldTerm::new_boxed(
            NameTerm::new_boxed("scope".to_string()),
            NameTerm::new_boxed("String".to_string()),
          ),
        ],
      ))],
    ))]);
  assert_eq!(ast, expected_ast);
}
