use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;
use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
pub struct ParamTerm {
  #[prop]
  name: Box<NameTerm>,
  #[prop]
  type_name: Box<NameTerm>,
}

impl ASTTerm for ParamTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_param(self)?;

    self.name.accept(visitor)?;
    self.type_name.accept(visitor)?;

    Ok(())
  }
}

impl ParamTerm {
  pub fn new_boxed(name: Box<NameTerm>, type_name: Box<NameTerm>) -> Box<ParamTerm> {
    Box::new(ParamTerm::new(name, type_name))
  }

  pub fn new(name: Box<NameTerm>, type_name: Box<NameTerm>) -> ParamTerm {
    ParamTerm { name, type_name }
  }
}
