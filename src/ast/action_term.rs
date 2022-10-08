use super::super::visitor::{Visitor, VisitorError};
use super::ast_term::ASTTerm;
use super::name_term::NameTerm;
use super::param_term::ParamTerm;
use super::returns_term::ReturnsTerm;

#[derive(PartialEq, Eq, Debug)]
pub struct ActionTerm {
  name: Box<NameTerm>,
  params: Vec<Box<ParamTerm>>,
  returned_type: Option<Box<ReturnsTerm>>,
}

impl ASTTerm for ActionTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_action(self)?;

    self.name.accept(visitor)?;
    for param in self.params.iter() {
      param.accept(visitor)?;
    }
    if let Some(ref returned_type) = self.returned_type {
      returned_type.accept(visitor)?;
    }

    Ok(())
  }
}

impl ActionTerm {
  pub fn name(&self) -> &NameTerm {
    &self.name
  }

  pub fn params(&self) -> &[Box<ParamTerm>] {
    &self.params
  }

  pub fn returned_type(&self) -> &Option<Box<ReturnsTerm>> {
    &self.returned_type
  }

  pub fn new_boxed(
    name: Box<NameTerm>,
    params: Vec<Box<ParamTerm>>,
    returned_type: Option<Box<ReturnsTerm>>,
  ) -> Box<ActionTerm> {
    Box::new(ActionTerm::new(name, params, returned_type))
  }

  pub fn new(
    name: Box<NameTerm>,
    params: Vec<Box<ParamTerm>>,
    returned_type: Option<Box<ReturnsTerm>>,
  ) -> ActionTerm {
    ActionTerm {
      name,
      params,
      returned_type,
    }
  }
}
