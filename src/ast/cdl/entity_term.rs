use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;
use super::super::common::reference::Reference;
use super::super::common::term_iter::TermIter;
use super::field_term::FieldTerm;
use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
pub struct EntityTerm {
  #[prop]
  name: Box<NameTerm>,
  applied_aspects: Vec<Reference<Box<NameTerm>>>,
  fields: Vec<Box<FieldTerm>>,
}

impl ASTTerm for EntityTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_entity(self)?;

    self.applied_aspects.accept(visitor)?;
    self.fields.accept(visitor)?;

    Ok(())
  }
}

impl EntityTerm {
  pub fn applied_aspects<'s>(&'s self) -> TermIter<'s, NameTerm> {
    TermIter::new_from_referenced_deref_vec(&self.applied_aspects)
  }

  pub fn fields<'s>(&'s self) -> TermIter<'s, FieldTerm> {
    TermIter::new_from_deref_vec(&self.fields)
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
    let aspects = applied_aspects
      .into_iter()
      .map(|x| Reference::new_fulfilled(x))
      .collect();
    EntityTerm {
      name,
      applied_aspects: aspects,
      fields,
    }
  }
}
