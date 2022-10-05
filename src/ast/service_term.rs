use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;
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

impl ASTTerm for ServiceTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_service(&self)?;
    self.name.accept(visitor)?;
    for definition in self.definitions.iter() {
      definition.accept(visitor)?;
    }
    Ok(())
  }
}

impl ServiceTerm {
  pub fn name(&self) -> &dyn ASTTerm {
    self.name.as_ref()
  }

  pub fn definitions(&self) -> &[Box<dyn ServiceUsableTerm>] {
    self.definitions.as_ref()
  }

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
