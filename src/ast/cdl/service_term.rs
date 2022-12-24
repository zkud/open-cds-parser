use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;
use super::action_term::ActionTerm;
use super::entity_term::EntityTerm;
use super::function_term::FunctionTerm;
use super::name_term::NameTerm;
use super::type_term::TypeTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(process_path = "process_service")]
pub struct ServiceTerm {
  #[subnode_prop]
  name: Box<NameTerm>,
  #[subnode_prop]
  definitions: Vec<ServiceDefinition>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ServiceDefinition {
  Entity(EntityTerm),
  Function(FunctionTerm),
  Action(ActionTerm),
  Type(TypeTerm),
}

impl ASTTerm for ServiceDefinition {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    match self {
      ServiceDefinition::Entity(entity) => entity.accept(visitor)?,
      ServiceDefinition::Function(function) => function.accept(visitor)?,
      ServiceDefinition::Action(action) => action.accept(visitor)?,
      ServiceDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
    };
    Ok(())
  }
}
