use super::super::visitor::{Visitor, VisitorError};
use std::fmt::Debug;
use std::ops::Deref;

pub trait ASTTerm: PartialEq + Eq + Debug {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError>;
}

impl<D, T> ASTTerm for D
where
  D: Deref<Target = T> + PartialEq + Eq + Debug,
  T: PartialEq + Eq + Debug + ASTTerm,
{
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    self.deref().accept(visitor)?;
    Ok(())
  }
}
