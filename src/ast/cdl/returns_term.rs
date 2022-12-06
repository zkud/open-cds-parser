use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;
use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
pub struct ReturnsTerm {
  #[prop]
  type_name: Box<NameTerm>,
  #[prop]
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
