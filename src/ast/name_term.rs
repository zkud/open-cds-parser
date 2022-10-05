use super::traits::ast_term::ASTTerm;
use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;

pub struct NameTerm {
  value: String,
}

impl ASTTerm for NameTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_name(&self)?;
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
