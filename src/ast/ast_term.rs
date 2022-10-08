use super::super::visitor::{Visitor, VisitorError};

pub trait ASTTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError>;
}
