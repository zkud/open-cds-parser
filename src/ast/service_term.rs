use super::super::visitor::{Visitor, VisitorError};
use super::action_term::ActionTerm;
use super::common::ast_term::ASTTerm;
use super::entity_term::EntityTerm;
use super::function_term::FunctionTerm;
use super::name_term::NameTerm;
use super::type_term::TypeTerm;

#[derive(PartialEq, Eq, Debug)]
pub struct ServiceTerm {
  name: Box<NameTerm>,
  definitions: Vec<ServiceDefinition>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ServiceDefinition {
  Entity(Box<EntityTerm>),
  Function(Box<FunctionTerm>),
  Action(Box<ActionTerm>),
  Type(Box<TypeTerm>),
}

impl ASTTerm for ServiceTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_service(self)?;
    self.name.accept(visitor)?;
    for definition in self.definitions.iter() {
      match definition {
        ServiceDefinition::Entity(entity) => entity.accept(visitor)?,
        ServiceDefinition::Function(function) => function.accept(visitor)?,
        ServiceDefinition::Action(action) => action.accept(visitor)?,
        ServiceDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
      };
    }
    Ok(())
  }
}

impl ServiceTerm {
  pub fn name(&self) -> &NameTerm {
    self.name.as_ref()
  }

  pub fn definitions(&self) -> &[ServiceDefinition] {
    self.definitions.as_ref()
  }

  pub fn new_boxed(name: Box<NameTerm>, definitions: Vec<ServiceDefinition>) -> Box<ServiceTerm> {
    Box::new(ServiceTerm::new(name, definitions))
  }

  pub fn new(name: Box<NameTerm>, definitions: Vec<ServiceDefinition>) -> ServiceTerm {
    ServiceTerm { name, definitions }
  }
}
