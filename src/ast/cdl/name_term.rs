use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;

#[derive(PartialEq, Eq, Debug)]
pub struct NameTerm {
  value: String,
}

impl ASTTerm for NameTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_name(self)?;
    Ok(())
  }
}

impl NameTerm {
  pub fn value(&self) -> &str {
    self.value.as_ref()
  }

  pub fn new_boxed(value: String) -> Box<NameTerm> {
    Box::new(NameTerm::new(value))
  }

  pub fn new(value: String) -> NameTerm {
    NameTerm { value }
  }
}
