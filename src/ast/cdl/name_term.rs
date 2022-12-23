use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
pub struct NameTerm {
  #[prop]
  value: String,
}

impl ASTTerm for NameTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_name(self)?;
    Ok(())
  }
}

impl NameTerm {
  pub fn new_boxed(value: String) -> Box<NameTerm> {
    Box::new(NameTerm::new(value))
  }

  pub fn new(value: String) -> NameTerm {
    NameTerm { value }
  }
}
