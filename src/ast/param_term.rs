use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;
use super::ast_term::ASTTerm;
use super::name_term::NameTerm;

#[derive(PartialEq, Eq, Debug)]
pub struct ParamTerm {
  name: Box<NameTerm>,
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
  pub fn name(&self) -> &NameTerm {
    self.name.as_ref()
  }

  pub fn type_name(&self) -> &NameTerm {
    self.type_name.as_ref()
  }

  pub fn new_boxed(name: Box<NameTerm>, type_name: Box<NameTerm>) -> Box<ParamTerm> {
    Box::new(ParamTerm::new(name, type_name))
  }

  pub fn new(name: Box<NameTerm>, type_name: Box<NameTerm>) -> ParamTerm {
    ParamTerm { name, type_name }
  }
}
