use super::traits::ast_term::ASTTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;

pub struct ActionTerm {
  name: Box<dyn ASTTerm>,
  params: Vec<Box<dyn ASTTerm>>,
  returned_type: Option<Box<dyn ASTTerm>>,
}

impl ASTTerm for ActionTerm {}

impl ServiceUsableTerm for ActionTerm {
  fn get_type(&self) -> ServiceTermType {
    ServiceTermType::Action
  }
}

impl ActionTerm {
  pub fn new_boxed(
    name: Box<dyn ASTTerm>,
    params: Vec<Box<dyn ASTTerm>>,
    returned_type: Option<Box<dyn ASTTerm>>,
  ) -> Box<ActionTerm> {
    Box::new(ActionTerm::new(name, params, returned_type))
  }

  pub fn new(
    name: Box<dyn ASTTerm>,
    params: Vec<Box<dyn ASTTerm>>,
    returned_type: Option<Box<dyn ASTTerm>>,
  ) -> ActionTerm {
    ActionTerm {
      name,
      params,
      returned_type,
    }
  }
}
