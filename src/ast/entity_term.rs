use super::super::visitor::{Visitor, VisitorError};
use super::ast_term::ASTTerm;
use super::field_term::FieldTerm;
use super::name_term::NameTerm;

#[derive(PartialEq, Eq, Debug)]
pub struct EntityTerm {
  name: Box<NameTerm>,
  applied_aspects: Vec<Box<NameTerm>>,
  fields: Vec<Box<FieldTerm>>,
}

impl ASTTerm for EntityTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_entity(self)?;

    for aspect in self.applied_aspects.iter() {
      aspect.accept(visitor)?;
    }
    for field in self.fields.iter() {
      field.accept(visitor)?;
    }

    Ok(())
  }
}

impl EntityTerm {
  pub fn name(&self) -> &NameTerm {
    &self.name
  }

  pub fn applied_aspects(&self) -> &[Box<NameTerm>] {
    &self.applied_aspects
  }

  pub fn fields(&self) -> &[Box<FieldTerm>] {
    &self.fields
  }

  pub fn new_boxed(
    name: Box<NameTerm>,
    applied_aspects: Vec<Box<NameTerm>>,
    fields: Vec<Box<FieldTerm>>,
  ) -> Box<EntityTerm> {
    Box::new(EntityTerm::new(name, applied_aspects, fields))
  }

  pub fn new(
    name: Box<NameTerm>,
    applied_aspects: Vec<Box<NameTerm>>,
    fields: Vec<Box<FieldTerm>>,
  ) -> EntityTerm {
    EntityTerm {
      name,
      applied_aspects,
      fields,
    }
  }
}
