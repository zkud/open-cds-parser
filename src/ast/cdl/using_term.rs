use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;
use super::name_term::NameTerm;
use super::super::common::reference::Reference;
use super::service_term::ServiceTerm;
use super::entity_term::EntityTerm;
use super::type_term::TypeTerm;

#[derive(PartialEq, Eq, Debug)]
pub struct UsingTerm {
  path: Box<NameTerm>,
  units: Vec<AliasedImportedUnit>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct AliasedImportedUnit {
  unit: Reference<ImportedUnit>,
  alias: Option<Box<NameTerm>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ImportedUnit {
  Entity(Box<EntityTerm>),
  Type(Box<TypeTerm>),
  Service(Box<ServiceTerm>),
}

impl ASTTerm for UsingTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_type(self)?;
    self.name.accept(visitor)?;
    self.resolved_type_name.accept(visitor)?;
    Ok(())
  }
}

impl UsingTerm {
  pub fn name(&self) -> &NameTerm {
    self.name.as_ref()
  }

  pub fn resolved_type_name(&self) -> &NameTerm {
    self.resolved_type_name.as_ref()
  }

  pub fn new_boxed(name: Box<NameTerm>, resolved_type_name: Box<NameTerm>) -> Box<TypeTerm> {
    Box::new(TypeTerm::new(name, resolved_type_name))
  }

  pub fn new(name: Box<NameTerm>, resolved_type_name: Box<NameTerm>) -> TypeTerm {
    TypeTerm {
      name,
      resolved_type_name,
    }
  }
}
