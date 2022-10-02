use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_usable_term::ServiceUsableTerm;

pub struct ServiceTerm {
  name: Box<dyn ASTTerm>,
  definitions: Vec<Box<dyn ServiceUsableTerm>>,
}

impl ModuleUsableTerm for ServiceTerm {
  fn get_type(&self) -> ModuleTermType {
    ModuleTermType::Service
  }
}

impl ASTTerm for ServiceTerm {}

impl ServiceTerm {
  pub fn new_boxed(
    name: Box<dyn ASTTerm>,
    definitions: Vec<Box<dyn ServiceUsableTerm>>,
  ) -> Box<ServiceTerm> {
    Box::new(ServiceTerm::new(name, definitions))
  }

  pub fn new(name: Box<dyn ASTTerm>, definitions: Vec<Box<dyn ServiceUsableTerm>>) -> ServiceTerm {
    ServiceTerm { name, definitions }
  }
}
