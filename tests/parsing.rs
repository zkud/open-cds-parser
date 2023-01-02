use open_cds_ast::ast::{
  EntityTerm, FieldTerm, FilePathTerm, ModuleDefinition, ModuleTerm, NameTerm, ServiceDefinition,
  ServiceTerm, UsingRefTerm, UsingTerm,
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

#[test]
fn with_correct_and_multi_modules_cds_it_buids_a_tree() {
  let parser = Parser::new("./tests/projects/modules/srv/cat-service.cds".to_string());

  let ast = parser.parse().unwrap();

  let expected_ast = Box::new(ModuleTerm::new(vec![
    ModuleDefinition::Using(UsingTerm::new(
      Box::new(FilePathTerm::new("../db/schema".to_string())),
      vec![],
    )),
    ModuleDefinition::Service(ServiceTerm::new(
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
    )),
  ]));
  assert_eq!(ast, expected_ast);
}
