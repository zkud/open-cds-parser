use super::super::visitor::{Visitor, VisitorError};
use super::common::ast_term::ASTTerm;
use super::name_term::NameTerm;

#[derive(PartialEq, Eq, Debug)]
pub struct ReturnsTerm {
  type_name: Box<NameTerm>,
  is_arrayed: bool,
}

impl ASTTerm for ReturnsTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_returns(self)?;
    self.type_name.accept(visitor)?;
    Ok(())
  }
}

impl ReturnsTerm {
  pub fn type_name(&self) -> &NameTerm {
    self.type_name.as_ref()
  }

  pub fn is_arrayed(&self) -> bool {
    self.is_arrayed
  }

  pub fn new_boxed(type_name: Box<NameTerm>) -> Box<ReturnsTerm> {
    Box::new(ReturnsTerm::new(type_name))
  }

  pub fn new(type_name: Box<NameTerm>) -> ReturnsTerm {
    ReturnsTerm {
      type_name,
      is_arrayed: false,
    }
  }

  pub fn new_arrayed_boxed(type_name: Box<NameTerm>) -> Box<ReturnsTerm> {
    Box::new(ReturnsTerm::new_arrayed(type_name))
  }

  pub fn new_arrayed(type_name: Box<NameTerm>) -> ReturnsTerm {
    ReturnsTerm {
      type_name,
      is_arrayed: true,
    }
  }
}
