use super::super::super::visitor::{Visitor, VisitorError};
use super::super::cdl::{EntityTerm, ServiceTerm, TypeTerm};
use super::super::common::ast_term::ASTTerm;
use super::using_term::UsingTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(visitor_path = "process_module")]
pub struct ModuleTerm {
  #[subnode_prop]
  definitions: Vec<ModuleDefinition>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ModuleDefinition {
  Using(UsingTerm),
  Entity(EntityTerm),
  Type(TypeTerm),
  Service(ServiceTerm),
}

impl ASTTerm for ModuleDefinition {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    match self {
      ModuleDefinition::Using(using) => using.accept(visitor)?,
      ModuleDefinition::Entity(entity) => entity.accept(visitor)?,
      ModuleDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
      ModuleDefinition::Service(service) => service.accept(visitor)?,
    };
    Ok(())
  }
}
