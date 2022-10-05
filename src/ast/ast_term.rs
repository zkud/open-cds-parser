use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;

pub trait ASTTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError>;
}
