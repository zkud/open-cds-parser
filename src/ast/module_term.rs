use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;
use super::ast_term::ASTTerm;
use super::entity_term::EntityTerm;
use super::service_term::ServiceTerm;
use super::type_term::TypeTerm;

#[derive(PartialEq, Eq, Debug)]
pub struct ModuleTerm {
  definitions: Vec<ModuleDefinition>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ModuleDefinition {
  Entity(Box<EntityTerm>),
  Type(Box<TypeTerm>),
  Service(Box<ServiceTerm>),
}

impl ASTTerm for ModuleTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_module(self)?;

    for param in self.definitions.iter() {
      match param {
        ModuleDefinition::Entity(entity) => entity.accept(visitor)?,
        ModuleDefinition::Type(type_declaration) => type_declaration.accept(visitor)?,
        ModuleDefinition::Service(service) => service.accept(visitor)?,
      };
    }

    Ok(())
  }
}

impl ModuleTerm {
  pub fn definitions(&self) -> &[ModuleDefinition] {
    self.definitions.as_ref()
  }

  pub fn new_boxed(definitions: Vec<ModuleDefinition>) -> Box<ModuleTerm> {
    Box::new(ModuleTerm::new(definitions))
  }

  pub fn new(definitions: Vec<ModuleDefinition>) -> ModuleTerm {
    ModuleTerm { definitions }
  }
}
