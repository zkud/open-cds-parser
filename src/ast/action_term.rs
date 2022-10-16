use super::super::visitor::{Visitor, VisitorError};
use super::common::ast_term::ASTTerm;
use super::common::term_iter::TermIter;
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
    self.params.accept(visitor)?;
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

  pub fn params<'s>(&'s self) -> TermIter<'s, ParamTerm> {
    TermIter::new_from_deref_vec(&self.params)
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
