use super::super::super::visitor::{Visitor, VisitorError};
use super::ast_term::ASTTerm;
use std::fmt::Debug;

/// A Suitable wrapper around the term to ast linkable and processable in parallel
#[derive(PartialEq, Eq, Debug)]
pub enum Reference<T: PartialEq + Eq + Debug> {
  Fulfilled(T),
  Virtual { qualifier: String },
}

impl<T: ASTTerm + PartialEq + Eq + Debug> ASTTerm for Reference<T> {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    if let Reference::Fulfilled(value) = self {
      value.accept(visitor)?;
      return Ok(());
    }
    panic!("Cannot accept visitor with a virtual reference")
  }
}

impl<T: PartialEq + Eq + Debug> Reference<T> {
  pub fn fulfill(&mut self, storage: &dyn ReferenceStorage<T>) {
    if let Reference::Virtual { qualifier } = self {
      *self = Reference::new_fulfilled(storage.get_value(qualifier))
    }
    panic!("Already fulfilled reference cannot be fulfilled")
  }
}

pub trait ReferenceStorage<T: PartialEq + Eq + Debug> {
  fn get_value(&self, qualifier: &str) -> T;
}

impl<T: PartialEq + Eq + Debug> Reference<T> {
  pub fn value(self) -> T {
    if let Reference::Fulfilled(value) = self {
      return value;
    }
    panic!("Cannot get value of a virtual reference")
  }

  pub fn value_ref(&self) -> &T {
    if let Reference::Fulfilled(value) = self {
      return value;
    }
    panic!("Cannot get value of a virtual reference")
  }

  pub fn value_ref_mut(&mut self) -> &mut T {
    if let Reference::Fulfilled(value) = self {
      return value;
    }
    panic!("Cannot get value of a virtual reference")
  }
}

impl<T: PartialEq + Eq + Debug> From<T> for Reference<T> {
  fn from(value: T) -> Reference<T> {
    Reference::Fulfilled(value)
  }
}

impl<T: PartialEq + Eq + Debug> Reference<T> {
  pub fn new_fulfilled(value: T) -> Reference<T> {
    Reference::Fulfilled(value)
  }

  pub fn new_virtual(qualifier: String) -> Reference<T> {
    Reference::Virtual { qualifier }
  }
}
