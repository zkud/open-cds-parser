use super::traits::ast_term::ASTTerm;
use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;

pub struct ReturnsTerm {
  type_name: Box<dyn ASTTerm>,
  is_arrayed: bool,
}

impl ASTTerm for ReturnsTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_returns(&self)?;
    self.type_name.accept(visitor)?;
    Ok(())
  }
}

impl ReturnsTerm {
  pub fn new_boxed(type_name: Box<dyn ASTTerm>) -> Box<ReturnsTerm> {
    Box::new(ReturnsTerm::new(type_name))
  }

  pub fn new(type_name: Box<dyn ASTTerm>) -> ReturnsTerm {
    ReturnsTerm {
      type_name,
      is_arrayed: false,
    }
  }

  pub fn new_arrayed_boxed(type_name: Box<dyn ASTTerm>) -> Box<ReturnsTerm> {
    Box::new(ReturnsTerm::new_arrayed(type_name))
  }

  pub fn new_arrayed(type_name: Box<dyn ASTTerm>) -> ReturnsTerm {
    ReturnsTerm {
      type_name,
      is_arrayed: true,
    }
  }
}
