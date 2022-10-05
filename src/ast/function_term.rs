use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;
use super::traits::ast_term::ASTTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;

pub struct FunctionTerm {
  name: Box<dyn ASTTerm>,
  params: Vec<Box<dyn ASTTerm>>,
  returned_type: Box<dyn ASTTerm>,
}

impl ASTTerm for FunctionTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_function(&self)?;

    self.name.accept(visitor)?;
    for param in self.params.iter() {
      param.accept(visitor)?;
    }
    self.returned_type.accept(visitor)?;

    Ok(())
  }
}

impl ServiceUsableTerm for FunctionTerm {
  fn get_type(&self) -> ServiceTermType {
    ServiceTermType::Function
  }
}

impl FunctionTerm {
  pub fn new_boxed(
    name: Box<dyn ASTTerm>,
    params: Vec<Box<dyn ASTTerm>>,
    returned_type: Box<dyn ASTTerm>,
  ) -> Box<FunctionTerm> {
    Box::new(FunctionTerm::new(name, params, returned_type))
  }

  pub fn new(
    name: Box<dyn ASTTerm>,
    params: Vec<Box<dyn ASTTerm>>,
    returned_type: Box<dyn ASTTerm>,
  ) -> FunctionTerm {
    FunctionTerm {
      name,
      params,
      returned_type,
    }
  }
}
