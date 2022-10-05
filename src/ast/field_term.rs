use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;
use super::traits::ast_term::ASTTerm;

pub struct FieldTerm {
  name: Box<dyn ASTTerm>,
  type_name: Box<dyn ASTTerm>,
}

impl ASTTerm for FieldTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_field(&self)?;

    self.name.accept(visitor)?;
    self.type_name.accept(visitor)?;

    Ok(())
  }
}

impl FieldTerm {
  pub fn name(&self) -> &Box<dyn ASTTerm> {
    &self.name
  }

  pub fn type_name(&self) -> &Box<dyn ASTTerm> {
    &self.type_name
  }

  pub fn new_boxed(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> Box<FieldTerm> {
    Box::new(FieldTerm::new(name, type_name))
  }

  pub fn new(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> FieldTerm {
    FieldTerm { name, type_name }
  }
}
