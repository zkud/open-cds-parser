use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;
use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
pub struct FieldTerm {
  #[sub_term]
  name: Box<NameTerm>,
  #[sub_term]
  type_name: Box<NameTerm>,
}

impl ASTTerm for FieldTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_field(self)?;

    self.name.accept(visitor)?;
    self.type_name.accept(visitor)?;

    Ok(())
  }
}

impl FieldTerm {
  pub fn new_boxed(name: Box<NameTerm>, type_name: Box<NameTerm>) -> Box<FieldTerm> {
    Box::new(FieldTerm::new(name, type_name))
  }

  pub fn new(name: Box<NameTerm>, type_name: Box<NameTerm>) -> FieldTerm {
    FieldTerm { name, type_name }
  }
}
