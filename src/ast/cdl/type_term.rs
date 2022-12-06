use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;
use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
pub struct TypeTerm {
  #[prop]
  name: Box<NameTerm>,
  #[prop]
  resolved_type_name: Box<NameTerm>,
}

impl ASTTerm for TypeTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_type(self)?;
    self.name.accept(visitor)?;
    self.resolved_type_name.accept(visitor)?;
    Ok(())
  }
}

impl TypeTerm {
  pub fn new_boxed(name: Box<NameTerm>, resolved_type_name: Box<NameTerm>) -> Box<TypeTerm> {
    Box::new(TypeTerm::new(name, resolved_type_name))
  }

  pub fn new(name: Box<NameTerm>, resolved_type_name: Box<NameTerm>) -> TypeTerm {
    TypeTerm {
      name,
      resolved_type_name,
    }
  }
}
