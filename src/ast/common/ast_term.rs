use super::super::super::visitor::{Visitor, VisitorError};
use std::ops::Deref;
use std::sync::Arc;

pub trait ASTTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError>;
}

impl<T: ASTTerm> ASTTerm for dyn Deref<Target = T> {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    self.deref().accept(visitor)?;
    Ok(())
  }
}

impl<T: ASTTerm> ASTTerm for Box<T> {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    self.deref().accept(visitor)?;
    Ok(())
  }
}

impl<T: ASTTerm> ASTTerm for Arc<T> {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    self.deref().accept(visitor)?;
    Ok(())
  }
}

impl<T: ASTTerm> ASTTerm for [T] {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    for term in self.iter() {
      term.accept(visitor)?;
    }
    Ok(())
  }
}
