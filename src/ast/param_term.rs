use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;
use super::traits::ast_term::ASTTerm;

pub struct ParamTerm {
  name: Box<dyn ASTTerm>,
  type_name: Box<dyn ASTTerm>,
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
  pub fn name(&self) -> &dyn ASTTerm {
    self.name.as_ref()
  }

  pub fn type_name(&self) -> &dyn ASTTerm {
    self.type_name.as_ref()
  }

  pub fn new_boxed(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> Box<ParamTerm> {
    Box::new(ParamTerm::new(name, type_name))
  }

  pub fn new(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> ParamTerm {
    ParamTerm { name, type_name }
  }
}
