use super::traits::ast_term::ASTTerm;

pub struct ReturnsTerm {
  type_name: Box<dyn ASTTerm>,
  is_arrayed: bool,
}

impl ASTTerm for ReturnsTerm {}

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
