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

impl<T: ASTTerm> ASTTerm for Option<T> {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    if let Some(variant) = self {
      variant.accept(visitor)?;
    }
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ast::LeafTerm;

  use std::sync::Arc;

  struct MockVisitor {
    pub visits: Vec<String>,
  }

  impl MockVisitor {
    pub fn new() -> Self {
      MockVisitor { visits: vec![] }
    }
  }

  #[derive(Debug, PartialEq)]
  struct VisitorError;

  impl Visitor for MockVisitor {
    fn process_mock_leaf(&mut self, term: &LeafTerm) -> Result<(), crate::visitor::VisitorError> {
      self.visits.push(term.value().clone());
      Ok(())
    }
  }

  #[test]
  fn test_deref_ast_term() {
    let term = Box::new(LeafTerm::new("1".to_string()));

    let mut visitor = MockVisitor::new();
    let result = term.accept(&mut visitor);

    assert!(result.is_ok());
    assert_eq!(visitor.visits, vec!["1"]);
  }

  #[test]
  fn test_option_ast_term_some() {
    let term = Some(LeafTerm::new("2".to_string()));

    let mut visitor = MockVisitor::new();
    let result = term.accept(&mut visitor);

    assert!(result.is_ok());
    assert_eq!(visitor.visits, vec!["2"]);
  }

  #[test]
  fn test_option_ast_term_none() {
    let term: Option<LeafTerm> = None;

    let mut visitor = MockVisitor::new();
    let result = term.accept(&mut visitor);

    assert!(result.is_ok());
    assert!(visitor.visits.is_empty());
  }

  #[test]
  fn test_box_ast_term() {
    let term = Box::new(LeafTerm::new("3".to_string()));

    let mut visitor = MockVisitor::new();
    let result = term.accept(&mut visitor);

    assert!(result.is_ok());
    assert_eq!(visitor.visits, vec!["3"]);
  }

  #[test]
  fn test_arc_ast_term() {
    let term = Arc::new(LeafTerm::new("4".to_string()));

    let mut visitor = MockVisitor::new();
    let result = term.accept(&mut visitor);

    assert!(result.is_ok());
    assert_eq!(visitor.visits, vec!["4"]);
  }

  #[test]
  fn test_slice_ast_term() {
    let terms = vec![
      LeafTerm::new("5".to_string()),
      LeafTerm::new("6".to_string()),
      LeafTerm::new("7".to_string()),
    ];

    let mut visitor = MockVisitor::new();
    let result = terms.as_slice().accept(&mut visitor);

    assert!(result.is_ok());
    assert_eq!(visitor.visits, vec!["5", "6", "7"]);
  }

  #[test]
  fn test_empty_slice_ast_term() {
    let terms: Vec<LeafTerm> = vec![];

    let mut visitor = MockVisitor::new();
    let result = terms.as_slice().accept(&mut visitor);

    assert!(result.is_ok());
    assert!(visitor.visits.is_empty());
  }
}
