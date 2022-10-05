use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;
use super::ast_term::ASTTerm;
use super::name_term::NameTerm;
use super::param_term::ParamTerm;
use super::returns_term::ReturnsTerm;

pub struct FunctionTerm {
  name: Box<NameTerm>,
  params: Vec<Box<ParamTerm>>,
  returned_type: Box<ReturnsTerm>,
}

impl ASTTerm for FunctionTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_function(self)?;

    self.name.accept(visitor)?;
    for param in self.params.iter() {
      param.accept(visitor)?;
    }
    self.returned_type.accept(visitor)?;

    Ok(())
  }
}

impl FunctionTerm {
  pub fn name(&self) -> &NameTerm {
    &self.name
  }

  pub fn params(&self) -> &[Box<ParamTerm>] {
    &self.params
  }

  pub fn returned_type(&self) -> &Box<ReturnsTerm> {
    &self.returned_type
  }

  pub fn new_boxed(
    name: Box<NameTerm>,
    params: Vec<Box<ParamTerm>>,
    returned_type: Box<ReturnsTerm>,
  ) -> Box<FunctionTerm> {
    Box::new(FunctionTerm::new(name, params, returned_type))
  }

  pub fn new(
    name: Box<NameTerm>,
    params: Vec<Box<ParamTerm>>,
    returned_type: Box<ReturnsTerm>,
  ) -> FunctionTerm {
    FunctionTerm {
      name,
      params,
      returned_type,
    }
  }
}
